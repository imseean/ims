use serde_json::Value;
use chrono::prelude::*;
use std::path::Path;
use std::fs::{remove_file, DirBuilder, File};
use serde_json;
use uuid::{Uuid, UuidVersion};
use std::io::*;
use super::Site;
use site::infrastructure::get_all_file;

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub create_date: DateTime<Utc>,
    #[serde(skip)] pub content: String,
    #[serde(skip)] pub path: String,
}

impl Content {
    fn create() -> Content {
        let content = Content {
            id: Uuid::new_v4(),
            title: "<title>".to_string(),
            description: "<description>".to_string(),
            tags: vec![],
            categories: vec![],
            create_date: Utc::now(),
            content: "<content>".to_string(),
            path: "".to_string(),
        };
        return content;
    }
    pub fn load(site: &Site, path: &str) -> Content {
        let file_path = Path::new(&site.root)
            .join(&site.content_directory)
            .join(path);
        if !file_path.exists() {
            panic!("The file is not exists.");
        }
        let file = File::open(file_path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
    }

    pub fn new(site: &Site, path: &str) -> Content {
        let file_path = Path::new(&site.root)
            .join(&site.content_directory)
            .join(path);

        if file_path.exists() {
            print!(
                "The file ({}) is exists. Do you wanna overwrite it.[Y/N]",
                path
            );
            stdout().flush();
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            if buffer.to_uppercase().starts_with("Y") {
                remove_file(&file_path).unwrap();
            } else {
                panic!("The file is exists.");
            }
        }
        let parent_path = file_path.parent().unwrap();
        if !parent_path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(parent_path)
                .unwrap();
        }

        let mut file = File::create(&file_path).unwrap();
        let mut content = Content::create();
        content.path = file_path.to_str().unwrap().to_string();
        let mark = serde_json::to_string_pretty(&content).unwrap().to_string();
        let data = format!("{}\n\r{}", mark, content.content);
        file.write_all(&mut data.into_bytes()).unwrap();
        return content;
    }

    pub fn find_all_content(site: &Site) -> Vec<String> {
        let parent_path = Path::new(&site.root).join(&site.content_directory);
        let list = get_all_file(&parent_path);
        let mut result = vec![];
        for item in &list {
            let mut path = Path::new(item);
            path = path.strip_prefix(parent_path.to_str().unwrap()).unwrap();
            result.push(path.to_str().unwrap().to_string());
        }
        return result;
    }
}
