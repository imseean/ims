mod content;
mod infrastructure;

use std;
use std::collections::HashMap;
use std::path::Path;
use std::fs::{remove_dir_all, DirBuilder, File};
use std::io::{Read, Write};

use serde_json::Value;
use serde_json;
use prettytable::{format, Table};
use chrono::{DateTime, Local, TimeZone};
use handlebars::Handlebars;

use self::content::Content;
use self::infrastructure::*;

type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub author: String,
    pub title: String,
    pub subtitle: String,
    pub baseurl: String,
    pub theme: String,
    pub theme_directory: String,
    pub content_directory: String,
    pub data_directory: String,
    pub build_directory: String,
    pub publish_directory: String,
    pub assets_directory: String,
    #[serde(skip)] pub root: String,
}

impl Site {
    /// Load site config from the specified directory.
    pub fn load(root_path: &str) -> Result<Site> {
        let root = root_path.to_string();
        let path = Path::new(root_path);
        if !path.exists() {
            return Err(Error::new("The dircetory is not exists."));
        }

        let config_path = path.join("config.json");
        let mut site = Site {
            name: "Ims".to_string(),
            author: "<author>".to_string(),
            title: "<title>".to_string(),
            subtitle: "<subtitle>".to_string(),
            baseurl: "<baseurl>".to_string(),
            theme: "default".to_string(),
            theme_directory: "theme".to_string(),
            content_directory: "content".to_string(),
            data_directory: "data".to_string(),
            build_directory: "build".to_string(),
            publish_directory: "publish".to_string(),
            assets_directory: "assets".to_string(),
            root: root,
        };
        if !config_path.exists() {
            return Err(Error::new("The config file is not exists."));
        }

        let mut config_file =
            File::open(config_path).map_err(|x| Error::new("Failed to open the config file."))?;
        let mut buffer = String::new();
        config_file
            .read_to_string(&mut buffer)
            .map_err(|x| Error::new("Failed to read the config file."))?;
        let value = serde_json::to_value(buffer)
            .map_err(|x| Error::new("Failed to resolve the config file."))?;
        site.name = value["name"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.name);
        site.author = value["author"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.author);
        site.title = value["title"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.title);
        site.subtitle = value["subtitle"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.subtitle);
        site.baseurl = value["baseurl"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.baseurl);
        site.theme = value["theme"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.theme);
        site.theme_directory = value["theme_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.theme_directory);
        site.content_directory = value["content_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.content_directory);
        site.data_directory = value["data_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.data_directory);
        site.build_directory = value["build_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.build_directory);
        site.publish_directory = value["publish_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.publish_directory);
        site.assets_directory = value["assets_directory"]
            .as_str()
            .map(|x| x.to_string())
            .unwrap_or(site.assets_directory);
        return Ok(site);
    }

    /// Create a new site with the specified directory.
    pub fn new(root_path: &str) -> Result<Site> {
        let root = root_path.to_string();
        let path = Path::new(root_path);
        if !path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(path)
                .map_err(|err| Error::new("Failed to create the site directory."))?;
        }
        let config_path = path.join("config.json");
        if config_path.exists() && config_path.is_file() {
            return Err(Error::new(
                "Failed to create a new site. because a site exists in the current directory.",
            ));
        }

        let mut site = Site {
            name: "Ims".to_string(),
            author: "<author>".to_string(),
            title: "<title>".to_string(),
            subtitle: "<subtitle>".to_string(),
            baseurl: "<baseurl>".to_string(),
            theme: "default".to_string(),
            theme_directory: "theme".to_string(),
            content_directory: "content".to_string(),
            data_directory: "data".to_string(),
            build_directory: "build".to_string(),
            publish_directory: "publish".to_string(),
            assets_directory: "assets".to_string(),
            root: root,
        };
        let mut config_file = File::create(config_path)
            .map_err(|err| Error::new("An error occurred while creating the config file."))?;
        let content = serde_json::to_string_pretty(&site)
            .map_err(|err| Error::new("An error occurred while converting the config content."))?;
        config_file
            .write(&content.into_bytes())
            .map_err(|err| Error::new("An error occurred while writing the config file."))?;
        return Ok(site);
    }

    /// Create a new Content.
    pub fn new_content(&self, content_path: &str) -> Result<()> {
        let content = Content::new(self, content_path).map_err(|err| {
            Error::new("An error occurred while creating content.").with_inner_error(&err)
        })?;
        Site::show_content_info(&content);
        return Ok(());
    }

    pub fn generate(&self) -> Result<()> {
        let layout_path = Path::new(&self.root)
            .join(&self.theme_directory)
            .join(&self.theme)
            .join("layout");
        let templates = get_all_file(&layout_path)
            .map_err(|err| Error::new("Failed to find template files.").with_inner_error(&err))?;
        let mut render = Handlebars::new();
        render.register_helper("json", Box::new(json_helper));
        render.register_helper("file", Box::new(file_helper));
        for template in &templates {
            let path = Path::new(template);
            if path.is_dir() {
                continue;
            }
            if path.extension()
                .ok_or(Error::new("Faild to get extension from the file."))? != "hbs"
            {
                continue;
            }
            let name = path.strip_prefix(layout_path
                .to_str()
                .ok_or(Error::new("Format of \"path\" is incorrect ."))?)
                .map_err(|err| {
                    Error::new("Failed to get the template name.").with_inner_error(&err)
                })?
                .to_str()
                .ok_or(Error::new("Format of \"path\" is incorrect ."))?;
            render.register_template_file(name, &path).map_err(|err| {
                Error::new("Failed to register the template.").with_inner_error(&err)
            })?;
        }
        let model = self.create_model()?;
        let data_path = Path::new(&self.root).join(&self.data_directory);
        if data_path.exists() {
            remove_dir_all(&data_path).map_err(|error| {
                Error::new("Failed to clear data directory.").with_inner_error(&error)
            })?;
        }
        for (key, _) in render.get_templates().clone().iter() {
            let map = match render.render_with_file(key, &model) {
                Ok(map) => map,
                Err(error) => {
                    warn!("{}", error);
                    continue;
                }
            };
            for (name, content) in map.iter() {
                let file_path = data_path.clone().join(name);
                {
                    let parent_path = file_path
                        .parent()
                        .ok_or(Error::new("Failed to get parent directory."))?;
                    DirBuilder::new()
                        .recursive(true)
                        .create(parent_path)
                        .map_err(|error| {
                            Error::new("An error occurred while creating the parent directory.")
                                .with_inner_error(&error)
                        })?;
                }

                let mut file = File::create(file_path).map_err(|error| {
                    Error::new("An error occurred while creating the file.")
                        .with_inner_error(&error)
                })?;
                file.write_all(&mut content.clone().into_bytes())
                    .map_err(|err| {
                        Error::new("An error occurred while save the file.").with_inner_error(&err)
                    })?;
            }
        }
        return Ok(());
    }
    pub fn publish(&self) {}
    pub fn server(&self) {}

    pub fn list_content(&self) -> Result<()> {
        let list: Vec<Content> = Content::load_all(self)?;
        let mut table = Table::new();
        table.set_titles(row![
            "TITLE",
            "CREATE DATE",
            "TAGS",
            "CATEGORIES",
            "PATH",
            "ID"
        ]);
        for item in &list {
            table.add_row(row![
                item.title,
                item.create_date.naive_local().format("%Y-%m-%d %H:%M:%S"),
                item.tags.join("/"),
                item.categories.join("/"),
                item.path,
                item.id
            ]);
        }
        // table.set_format(*format::consts::FORMAT_DEFAULT);
        table.printstd();
        return Ok(());
    }

    fn create_model(&self) -> Result<Value> {
        let contents = Content::load_all(self)?;
        let mut tags: HashMap<String, Vec<&Content>> = HashMap::new();
        let mut categories: HashMap<String, Vec<&Content>> = HashMap::new();
        for content in &contents {
            for tag in &content.tags {
                if tags.contains_key(tag) {
                    tags.get_mut(tag).unwrap().push(content);
                } else {
                    tags.insert(tag.clone(), vec![content]);
                }
            }
            for category in &content.categories {
                if categories.contains_key(category) {
                    categories.get_mut(category).unwrap().push(content);
                } else {
                    categories.insert(category.clone(), vec![content]);
                }
            }
        }
        return Ok(json!({
            "site":self,
            "contents":&contents,
            "tags":tags,
            "categories":categories
        }));
    }

    fn show_content_info(content: &Content) {
        let mut table = Table::new();
        table.set_titles(row![
            "TITLE",
            "CREATE DATE",
            "TAGS",
            "CATEGORIES",
            "PATH",
            "ID"
        ]);
        table.add_row(row![
            content.title,
            content
                .create_date
                .naive_local()
                .format("%Y-%m-%d %H:%M:%S"),
            content.tags.join("/"),
            content.categories.join("/"),
            content.path,
            content.id
        ]);
        table.printstd();
    }
}
