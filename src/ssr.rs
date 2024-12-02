use std::{env::current_exe, path::PathBuf, sync::Arc};

use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    Extension,
};
use serde_json::Value;
use tokio::{fs, process::Command};

const ENTRY_SERVER_JS_RENDER: &str = r#"(async () => {
  const { render } = await import(process.argv[2]);
  const url = process.argv[3];
  const result = render(url);
  console.log(JSON.stringify(result));
})();
"#;

pub struct Rendered {
    head: String,
    html: String,
}

pub struct AppState {
    // is_production: bool,
    pub template_html: Option<String>,
    // ssr_manifest: Option<Value>,
    pub entry_server_render_cjs_path: PathBuf,
    pub entry_server_path: PathBuf,
}

pub async fn setup_app_state_ssr() -> Arc<AppState> {
    let template_html = Some(
        fs::read_to_string("./client/dist/client/index.html")
            .await
            .expect("Template file missing"),
    );

    fs::write(
        current_exe()
            .expect("Failed to get current exe path: {e}")
            .parent()
            .unwrap()
            .join("entry-server-render.cjs"),
        ENTRY_SERVER_JS_RENDER,
    )
    .await
    .expect("Failed to write entry-server-render.cjs");

    let entry_server_render_cjs_path = current_exe()
        .expect("Failed to get current exe path: {e}")
        .parent()
        .unwrap()
        .join("entry-server-render.cjs");

    let entry_server_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("client/dist/server/entry-server.js");

    Arc::new(AppState {
        template_html,
        entry_server_render_cjs_path,
        entry_server_path,
    })
}

pub async fn handle_ssr(
    Extension(app_state): Extension<Arc<AppState>>,
    uri: Uri,
) -> impl IntoResponse {
    let url = uri.path().to_string();

    let template = app_state.template_html.clone().unwrap();
    let rendered = execute_ssr(&url, app_state.as_ref()).await;
    let html = template
        .replace("<!--app-html-->", &rendered.html)
        .replace("<!--app-head-->", &rendered.head);
    Html(html)
}

async fn execute_ssr(url: &str, app_state: &AppState) -> Rendered {
    let output = Command::new("node")
        .arg(app_state.entry_server_render_cjs_path.clone())
        .arg(app_state.entry_server_path.clone())
        .arg(url)
        .output()
        .await
        .expect("failed to execute SSR script");

    let rendered = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    let json: Value = serde_json::from_str(&rendered).expect("Failed to parse JSON");

    let head = json["head"].as_str().unwrap().to_string();
    let html = json["html"].as_str().unwrap().to_string();

    Rendered { head, html }
}
