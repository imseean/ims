use std;

use serde_json::{self, Value};
use chrono::prelude::*;
use uuid::Uuid;
use regex::Regex;

use super::Site;
use super::super::infrastructure::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_draft: bool,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub create_date: DateTime<Utc>,
    #[serde(skip)] pub content: String,
    #[serde(skip)] pub path: String,
}

impl Content {
    fn default() -> Content {
        let content = Content {
            id: Uuid::new_v4(),
            title: "<title>".to_string(),
            description: "<description>".to_string(),
            is_draft: true,
            tags: vec![],
            categories: vec![],
            create_date: Utc::now(),
            content: "Content".to_string(),
            path: "".to_string(),
        };
        return content;
    }
    /// Load Content from exists file.
    pub fn load(_: &Site, path: &str, data: &str) -> Result<Content> {
        let re = Regex::new(
            r"^\s*``````` json(?P<description>(.|\s)*?)```````(?P<content>(.|\s)*)",
        ).map_err(|err| {
            Error::new("An error occurred while resolving the content.").with_inner_error(&err)
        })?;

        let caps = re.captures(&data).ok_or(Error::new(
            "Failed to find description info on the content.",
        ))?;

        let value: Value = serde_json::from_str(&caps["description"]).map_err(|err| {
            Error::new("An error occurred while resolving description of the content.")
                .with_inner_error(&err)
        })?;
        let mut content = Content::default();
        let id = Uuid::parse_str(value["id"]
            .as_str()
            .ok_or(Error::new("\"id\" is required."))?)
            .map_err(|err| {
            Error::new("The format of \"id\" is incorrect.").with_inner_error(&err)
        })?;
        content.id = id;
        content.title = value["title"]
            .as_str()
            .map(|x| x.to_string())
            .ok_or(Error::new("\"title\" is required."))?;
        content.description = value["description"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(String::new());
        content.tags = value["tags"]
            .as_array()
            .map(|x| {
                x.iter()
                    .map(|x| x.as_str().map(|x| x.to_string()).unwrap_or(String::new()))
                    .collect()
            })
            .ok_or(Error::new(
                "An error occurred while resolving \"tags\" of the description.",
            ))?;
        content.categories = value["categories"]
            .as_array()
            .map(|x| {
                x.iter()
                    .map(|x| x.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .ok_or(Error::new(
                "An error occurred while resolving \"categories\" of the description.",
            ))?;
        let create_date = value["create_date"]
            .as_str()
            .ok_or(Error::new("\"create_date\" is required."))?
            .parse::<DateTime<Utc>>()
            .map_err(|err| {
                Error::new("The format of \"create_date\" is incorrect.").with_inner_error(&err)
            })?;
        content.create_date = create_date;
        content.content = caps["content"].to_string();
        content.path = path.to_string();
        return Ok(content);
    }

    /// Create a new Content,and save it to file.
    pub fn new(_: &Site, path: &str) -> Result<Content> {
        let mut content = Content::default();
        content.path = path.to_string();
        return Ok(content);
    }
}
