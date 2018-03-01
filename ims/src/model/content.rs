use std;
use std::io;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

use serde_json::{self, Value};
use chrono::prelude::*;
use uuid::Uuid;
use regex::Regex;
use colored::*;
use prettytable::{format, Table};

use super::Site;
use super::super::infrastructure::{get_all_file, Error};

type Result<T> = std::result::Result<T, Error>;

fn default_id() -> Uuid {
    Uuid::new_v4()
}
fn default_title() -> String {
    "TITLE".to_string()
}
fn default_description() -> String {
    "DESCRIPTION".to_string()
}

fn default_target() -> String {
    "DRAFT".to_string()
}
fn default_tags() -> Vec<String> {
    vec![]
}
fn default_create_time() -> DateTime<Utc> {
    Utc::now()
}
fn default_meta() -> Value {
    Value::Null
}
fn default_content() -> String {
    "# Content \r\nmarkdown document.".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    #[serde(default = "default_id")] pub id: Uuid,
    #[serde(default = "default_title")] pub title: String,
    #[serde(default = "default_description")] pub description: String,
    #[serde(default = "default_target")] pub target: String,
    #[serde(default = "default_tags")] pub tags: Vec<String>,
    #[serde(default = "default_create_time")] pub create_time: DateTime<Utc>,
    #[serde(default = "default_meta")] pub meta: Value,
    #[serde(skip_deserializing, default = "default_content")] pub content: String,
    #[serde(skip)] pub path: String,
}

impl Content {
    fn default() -> Content {
        let content = serde_json::from_str::<Content>("{}").unwrap();
        return content;
    }

    /// Create a new Content,and save it to file.
    pub fn new(site: &Site, path: &str) -> Result<Content> {
        println!(
            "{0:>12} {1} {2}",
            "Creating".green().bold(),
            "content",
            path
        );
        let content_path = site.get_content_path()?;
        let file_path = Path::new(&content_path).join(&path);

        if file_path.exists() {
            print!(
                "File ({}) already exists. Type [Y] to overwrite it .[Y/N]",
                path
            );
            io::stdout().flush().map_err(|err| {
                Error::new("An error occurred while creating content.").with_inner_error(&err)
            })?;
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).map_err(|err| {
                Error::new("An error occurred while creating content.").with_inner_error(&err)
            })?;
            if buffer.to_uppercase().starts_with("Y") {
                fs::remove_file(&file_path).map_err(|err| {
                    Error::new("An error occurred while creating content.").with_inner_error(&err)
                })?;
            } else {
                return Err(Error::new("File already exists."));
            }
        }
        let parent_path = file_path.parent().ok_or(Error::new(
            "An error occurred while getting parent directory from path.",
        ))?;
        if !parent_path.exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(parent_path)
                .map_err(|err| {
                    Error::new("An error occurred while creating parent directory.")
                        .with_inner_error(&err)
                })?;
        }

        let mut file = fs::File::create(&file_path).map_err(|err| {
            Error::new("An error occurred while creating file.").with_inner_error(&err)
        })?;
        let mut content = Content::default();
        content.path = path.to_string();
        let mut value = serde_json::to_value(content.clone()).unwrap();
        let map = value.as_object_mut().unwrap();
        map.remove("content").unwrap();
        let mark = serde_json::to_string_pretty(&map)
            .map_err(|err| Error::new("An error occurred while save file.").with_inner_error(&err))?
            .to_string();
        let data = format!("``````` json\r\n{}\r\n```````\r\n{}", mark, content.content);
        file.write_all(&mut data.into_bytes()).map_err(|err| {
            Error::new("An error occurred while save file.").with_inner_error(&err)
        })?;

        return Ok(content);
    }

    pub fn list(site: &Site) -> Result<()> {
        let mut contents = Self::load_all(site)?;
        contents.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        let mut table = Table::new();
        table.set_titles(row!["TITLE", "CREATE DATE", "ID"]);
        for item in &contents {
            table.add_row(row![
                item.title,
                item.create_time.naive_local().format("%Y-%m-%d %H:%M:%S"),
                item.id
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();
        return Ok(());
    }

    /// Load Content from exists file.
    pub fn load(site: &Site, path: &str) -> Result<Content> {
        trace!("Loading content {}", path);
        let content_path = site.get_content_path()?;
        let file_path = Path::new(&content_path).join(path);
        if !file_path.exists() {
            return Err(Error::new("The file is not exists."));
        }
        let mut file = fs::File::open(file_path)
            .map_err(|err| Error::new("Failed to open the file.").with_inner_error(&err))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|err| Error::new("Failed to read file.").with_inner_error(&err))?;

        let re = Regex::new(r"^\s*``````` json(?P<mark>(.|\s)*?)```````(?P<content>(.|\s)*)")
            .map_err(|err| {
                Error::new("An error occurred while resolving the content.").with_inner_error(&err)
            })?;

        let caps = re.captures(&buffer)
            .ok_or(Error::new("Failed to find mark info on the content."))?;

        let mut content = serde_json::from_str::<Content>(&caps["mark"]).map_err(|error| {
            Error::new("Failed to convert mark info on the content.").with_inner_error(&error)
        })?;

        content.content = caps["content"].to_string();
        content.path = path.to_string();
        return Ok(content);
    }

    pub fn load_all(site: &Site) -> Result<Vec<Content>> {
        trace!("Loading contents");
        let content_path = site.get_content_path()?;
        let parent_path = Path::new(&content_path);
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
        let mut contents = vec![];
        for path in paths {
            let content = match Self::load(&site, &path) {
                Ok(content) => content,
                Err(err) => {
                    warn!("Failed to load content:{}. error:{}", path, err);
                    continue;
                }
            };
            contents.push(content);
        }
        trace!("Loaded {} content(s)", contents.len());
        return Ok(contents);
    }
}
