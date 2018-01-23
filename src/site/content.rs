use std;
use std::io::*;
use std::path::Path;
use std::fs::{remove_file, DirBuilder, File};

use serde_json::{self, Value};
use chrono::prelude::*;
use uuid::Uuid;
use regex::Regex;

use super::Site;
use super::infrastructure::{get_all_file, Error};

type Result<T> = std::result::Result<T, Error>;

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
            content: "Content".to_string(),
            path: "".to_string(),
        };
        return content;
    }
    /// Load Content from exists file.
    pub fn load(site: &Site, path: &str) -> Result<Content> {
        let file_path = Path::new(&site.root)
            .join(&site.content_directory)
            .join(path);
        if !file_path.exists() {
            return Err(Error::new("The file is not exists."));
        }
        let mut file = File::open(file_path)
            .map_err(|err| Error::new("Failed to open the file.").with_inner_error(&err))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| Error::new("Failed to read file.").with_inner_error(&err))?;
        let re = Regex::new(
            r"^\s*``````` json(?P<description>(.|\s)*?)```````(?P<content>(.|\s)*)",
        ).map_err(|err| {
            Error::new("An error occurred while resolving the content.").with_inner_error(&err)
        })?;

        let caps = re.captures(&buffer).ok_or(Error::new(
            "Failed to find description info on the content.",
        ))?;

        let value: Value = serde_json::from_str(&caps["description"]).map_err(|err| {
            Error::new("An error occurred while resolving description of the content.")
                .with_inner_error(&err)
        })?;
        let mut content = Content::create();
        content.path = path.to_string();
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
        let create_date = create_date;

        content.create_date = create_date;
        content.content = caps["content"].to_string();
        content.path = path.to_string();
        return Ok(content);
    }

    /// Create a new Content,and save it to file.
    pub fn new(site: &Site, path: &str) -> Result<Content> {
        let file_path = Path::new(&site.root)
            .join(&site.content_directory)
            .join(path);

        if file_path.exists() {
            print!(
                "The file ({}) already exists. Type [Y] to overwrite it .[Y/N]",
                path
            );
            stdout().flush().map_err(|err| {
                Error::new("An error occurred while creating content.").with_inner_error(&err)
            })?;
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).map_err(|err| {
                Error::new("An error occurred while creating content.").with_inner_error(&err)
            })?;
            if buffer.to_uppercase().starts_with("Y") {
                remove_file(&file_path).map_err(|err| {
                    Error::new("An error occurred while creating content.").with_inner_error(&err)
                })?;
            } else {
                return Err(Error::new("The file already exists."));
            }
        }
        let parent_path = file_path.parent().ok_or(Error::new(
            "An error occurred while getting the parent directory from the path.",
        ))?;
        if !parent_path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(parent_path)
                .map_err(|err| {
                    Error::new("An error occurred while creating the parent directory.")
                        .with_inner_error(&err)
                })?;
        }

        let mut file = File::create(&file_path).map_err(|err| {
            Error::new("An error occurred while creating the file.").with_inner_error(&err)
        })?;
        let mut content = Content::create();
        content.path = file_path
            .to_str()
            .ok_or(Error::new("Format of \"path\" is incorrect ."))?
            .to_string();
        let mark = serde_json::to_string_pretty(&content)
            .map_err(|err| {
                Error::new("An error occurred while save the file.").with_inner_error(&err)
            })?
            .to_string();
        let data = format!("``````` json\r\n{}\r\n```````\r\n{}", mark, content.content);
        file.write_all(&mut data.into_bytes()).map_err(|err| {
            Error::new("An error occurred while save the file.").with_inner_error(&err)
        })?;
        return Ok(content);
    }

    /// Load all Contents from the content directory of the site.
    /// If any content load failed, it's will show a warning.
    pub fn load_all(site: &Site) -> Result<Vec<Content>> {
        let paths = Content::get_all_content_path(site)?;
        let mut contents = vec![];
        for path in paths {
            let content = match Content::load(site, &path) {
                Ok(content) => content,
                Err(err) => {
                    warn!("Failed to load content:{}.error:{}", path, err);
                    continue;
                }
            };
            contents.push(content);
        }
        return Ok(contents);
    }

    pub fn get_all_content_path(site: &Site) -> Result<Vec<String>> {
        let parent_path = Path::new(&site.root).join(&site.content_directory);
        let list = get_all_file(&parent_path)?;
        let mut paths = vec![];
        for item in &list {
            let mut path = Path::new(item);
            path = path.strip_prefix(parent_path
                .to_str()
                .ok_or(Error::new(&format!("Format of \"path\" is incorrect.")))?)
                .map_err(|err| {
                    Error::new(&format!(
                        "The Path is not The child path of the parent path."
                    )).with_inner_error(&err)
                })?;
            paths.push(
                path.to_str()
                    .ok_or(Error::new(&format!("Format of \"path\" is incorrect.")))?
                    .to_string(),
            );
        }
        return Ok(paths);
    }
}
