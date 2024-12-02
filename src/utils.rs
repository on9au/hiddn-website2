use std::collections::HashMap;

use chrono::{DateTime, Utc};
use tokio::fs;

use crate::payloads::AnnouncementPayload;

pub async fn load_docs() -> HashMap<String, HashMap<String, String>> {
    let mut docs = HashMap::new();

    let mut paths = fs::read_dir("./docs").await.unwrap();

    let mut dir_entries = Vec::new();
    while let Some(entry) = paths.next_entry().await.unwrap() {
        dir_entries.push(entry);
    }

    for entry in dir_entries {
        println!("{:?}", entry.file_name());
        let os_name = entry.file_name().to_string_lossy().to_lowercase();
        let mut os_docs = HashMap::new();

        let os_path = entry.path();
        let mut files = fs::read_dir(os_path).await.unwrap();

        let mut file_entries = Vec::new();
        while let Some(file) = files.next_entry().await.unwrap() {
            file_entries.push(file);
        }

        for file in file_entries {
            println!("{:?}", file.file_name());
            let file_name = file.file_name().to_string_lossy().to_lowercase();
            if file_name.ends_with(".md") {
                let category = file_name.trim_end_matches(".md").to_string();
                let content = fs::read_to_string(file.path()).await.unwrap_or_default();
                os_docs.insert(category, content);
            }
        }

        docs.insert(os_name, os_docs);
    }

    println!("{:?}", docs);

    docs
}

pub async fn load_announcements() -> Vec<AnnouncementPayload> {
    let mut announcements = Vec::new();

    // Read announcements dir
    let mut paths = fs::read_dir("./announcements").await.unwrap();

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

    entries_with_metadata.sort_by_key(|&(_, modified)| std::cmp::Reverse(modified));

    let dir_entries: Vec<_> = entries_with_metadata
        .into_iter()
        .map(|(entry, _)| entry)
        .collect();

    for entry in dir_entries {
        println!("{:?}", entry.file_name());
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        if file_name.ends_with(".md") {
            let content = fs::read_to_string(entry.path()).await.unwrap_or_default();
            let announcement = AnnouncementPayload {
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

    println!("{:?}", announcements);

    announcements
}
