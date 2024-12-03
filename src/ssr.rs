use std::sync::Arc;

use axum::{
    http::Uri,
    response::{Html, IntoResponse},
    Extension,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{fs, process::Child};
use tracing::info;

use crate::config::GLOBAL_CONFIG;

pub struct Rendered {
    head: String,
    html: String,
}

#[derive(Serialize)]
struct RenderRequest {
    url: String,
}

#[derive(Deserialize)]
struct RenderResponse {
    head: Option<String>,
    html: Option<String>,
}

pub struct AppState {
    pub template_html: Option<String>,
    pub node_runtime: Child,
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.node_runtime
            .start_kill()
            .expect("Failed to kill SSR server");
    }
}

pub async fn setup_app_state_ssr() -> Arc<AppState> {
    let template_html = Some(
        fs::read_to_string("./client/dist/client/index.html")
            .await
            .expect("Template file missing"),
    );

    let node_runtime = tokio::process::Command::new("node")
        .arg("./client/server-ssr.js")
        .arg(GLOBAL_CONFIG.express_port.to_string())
        .spawn()
        .expect("Failed to start SSR server");

    info!("SSR server listening at {}", GLOBAL_CONFIG.express_port);

    Arc::new(AppState {
        template_html,
        node_runtime,
    })
}

pub async fn handle_ssr(
    Extension(app_state): Extension<Arc<AppState>>,
    uri: Uri,
) -> impl IntoResponse {
    let url = uri.path().to_string();

    let template = app_state.template_html.clone().unwrap();
    let rendered = execute_ssr(&url).await;
    let html = template
        .replace("<!--app-html-->", &rendered.html)
        .replace("<!--app-head-->", &rendered.head);
    Html(html)
}

async fn execute_ssr(url: &str) -> Rendered {
    let client = Client::new();
    let response = client
        .post("http://localhost:3001/render")
        .json(&RenderRequest {
            url: url.to_string(),
        })
        .send()
        .await
        .expect("Failed to send request to SSR server");

    if !response.status().is_success() {
        panic!("SSR server returned an error");
    }

    let rendered: RenderResponse = response
        .json()
        .await
        .expect("Failed to parse SSR server response");

    Rendered {
        head: rendered.head.unwrap_or("".to_owned()),
        html: rendered.html.unwrap_or("".to_owned()),
    }
}
