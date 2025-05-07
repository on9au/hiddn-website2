use chrono::{DateTime, Utc};
use tokio::fs;

use crate::{config::GLOBAL_CONFIG, payloads::Announcement};

pub async fn load_announcements() -> Vec<Announcement> {
    let mut announcements = Vec::new();

    // Read announcements dir
    let mut paths = fs::read_dir(GLOBAL_CONFIG.announcements_dir.clone())
        .await
        .unwrap();

    let mut dir_entries = Vec::new();

    while let Some(entry) = paths.next_entry().await.unwrap() {
        dir_entries.push(entry);
    }

    let mut entries_with_metadata: Vec<_> =
        futures::future::join_all(dir_entries.into_iter().map(|entry| async {
            let metadata = entry.metadata().await.unwrap();
            let modified = metadata.modified().unwrap();
            (entry, modified)
        }))
        .await;

    entries_with_metadata.sort_by_key(|&(_, modified)| modified);

    let dir_entries: Vec<_> = entries_with_metadata
        .into_iter()
        .map(|(entry, _)| entry)
        .collect();

    for entry in dir_entries {
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        if file_name.ends_with(".md") {
            let content = fs::read_to_string(entry.path()).await.unwrap_or_default();
            let announcement = Announcement {
                id: (announcements.len() as u32).into(),
                title: file_name.trim_end_matches(".md").to_string(),
                date: DateTime::<Utc>::from(entry.metadata().await.unwrap().modified().unwrap())
                    .to_rfc3339()
                    .to_string(),
                content,
            };
            announcements.push(announcement);
        }
    }

    announcements
}
