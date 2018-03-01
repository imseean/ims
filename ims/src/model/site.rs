use std;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use serde_json::{self, Value};
use handlebars::Handlebars;
use colored::*;
use iron::prelude::*;
use staticfile::Static;
use mount::Mount;

pub use super::*;
use super::super::infrastructure::*;

type Result<T> = std::result::Result<T, Error>;

fn default_title() -> String {
    "<Title>".to_string()
}
fn default_author() -> String {
    "<Author>".to_string()
}
fn default_subtitle() -> String {
    "<Subtitle>".to_string()
}
fn default_address() -> String {
    "/".to_string()
}
fn default_theme() -> String {
    "default".to_string()
}
fn default_meta() -> Value {
    Value::Null
}
fn default_theme_directory() -> String {
    "theme".to_string()
}
fn default_content_directory() -> String {
    "content".to_string()
}
fn default_build_directory() -> String {
    "build".to_string()
}
fn default_data_directory() -> String {
    "data".to_string()
}
fn default_publish_directory() -> String {
    "publish".to_string()
}
fn default_root() -> String {
    ".".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    #[serde(default = "default_title")] pub title: String,
    #[serde(default = "default_author")] pub author: String,
    #[serde(default = "default_subtitle")] pub subtitle: String,
    #[serde(default = "default_address")] pub address: String,
    #[serde(default = "default_theme")] pub theme: String,
    #[serde(default = "default_meta")] pub meta: Value,
    #[serde(default = "default_theme_directory")] pub theme_directory: String,
    #[serde(default = "default_content_directory")] pub content_directory: String,
    #[serde(default = "default_data_directory")] pub data_directory: String,
    #[serde(default = "default_build_directory")] pub build_directory: String,
    #[serde(default = "default_publish_directory")] pub publish_directory: String,
    #[serde(skip_serializing, default = "default_root")] pub root: String,
}

impl Site {
    pub fn get_theme_path(&self) -> Result<String> {
        let path = Path::new(&self.root)
            .join(&self.theme_directory)
            .join(&self.theme);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get theme path."))?
            .to_string();
        return Ok(path);
    }
    pub fn get_content_path(&self) -> Result<String> {
        let path = Path::new(&self.root).join(&self.content_directory);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get content path."))?
            .to_string();
        return Ok(path);
    }
    #[allow(dead_code)]
    pub fn get_data_path(&self) -> Result<String> {
        let path = Path::new(&self.root).join(&self.data_directory);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get data path."))?
            .to_string();
        return Ok(path);
    }
    pub fn get_build_path(&self) -> Result<String> {
        let path = Path::new(&self.root).join(&self.build_directory);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get build path."))?
            .to_string();
        return Ok(path);
    }
    pub fn get_publish_path(&self) -> Result<String> {
        let path = Path::new(&self.root).join(&self.publish_directory);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get publish path."))?
            .to_string();
        return Ok(path);
    }

    /// Create a new site with the specified directory.
    pub fn new(root_path: &str) -> Result<Site> {
        println!(
            "{0:>12} {1} {2}",
            "Creating".green().bold(),
            "site",
            root_path
        );
        let path = Path::new(root_path);
        if !path.exists() {
            fs::DirBuilder::new()
                .recursive(true)
                .create(path)
                .map_err(|err| {
                    Error::new("Failed to create the site directory.").with_inner_error(&err)
                })?;
        }
        let config_path = path.join("config.json");
        if config_path.exists() && config_path.is_file() {
            return Err(Error::new(
                "Failed to create a new site. because a site exists in the current directory.",
            ));
        }
        let mut site = serde_json::from_str::<Site>("{}").unwrap();
        site.root = root_path.to_string();
        let mut config_file = fs::File::create(config_path).map_err(|err| {
            Error::new("An error occurred while creating the config file.").with_inner_error(&err)
        })?;
        let content = serde_json::to_string_pretty(&site).map_err(|err| {
            Error::new("An error occurred while converting the config content.")
                .with_inner_error(&err)
        })?;
        config_file.write(&content.into_bytes()).map_err(|err| {
            Error::new("An error occurred while writing the config file.").with_inner_error(&err)
        })?;

        return Ok(site);
    }

    /// Load site config from the specified directory.
    pub fn load(root_path: &str) -> Result<Site> {
        trace!("Loading site {0}", root_path);
        let path = Path::new(root_path);
        if !path.exists() {
            return Err(Error::new("The dircetory is not exists."));
        }
        let config_path = path.join("config.json");
        if !config_path.exists() {
            return Err(Error::new("The config file is not exists."));
        }

        let mut config_file = fs::File::open(config_path).map_err(|error| {
            Error::new("Failed to open the config file.").with_inner_error(&error)
        })?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer).map_err(|error| {
            Error::new("Failed to read the config file.").with_inner_error(&error)
        })?;

        let mut site = serde_json::from_str::<Site>(&buffer).map_err(|error| {
            Error::new("Failed to resolve the config file.").with_inner_error(&error)
        })?;
        site.root = root_path.to_string();
        return Ok(site);
    }

    pub fn info(&self) -> Result<()> {
        println!("{0:>12}:\"{1}\"", "Title".bold(), self.title);
        println!("{0:>12}:\"{1}\"", "Subtitle".bold(), self.subtitle);
        println!("{0:>12}:\"{1}\"", "Author".bold(), self.author);
        println!("{0:>12}:\"{1}\"", "Address".bold(), self.address);
        println!("{0:>12}:\"{1}\"", "Theme".bold(), self.theme);
        return Ok(());
    }

    pub fn build(&self) -> Result<()> {
        println!(
            "{0:>12} {1} {2}",
            "Building".green().bold(),
            "site",
            self.root
        );
        let model = self.create_model()?;
        let build_path = self.get_build_path()?;
        let data_path = Path::new(&build_path);
        if data_path.exists() {
            fs::remove_dir_all(&data_path).map_err(|error| {
                Error::new("Failed to clear data directory.").with_inner_error(&error)
            })?;
        }
        let mut render = self.create_render()?;
        for (key, _) in render.get_templates().clone().iter() {
            trace!("Rendering template:{}", key);
            let map = match render.render_with_file(key, &model) {
                Ok(map) => map,
                Err(error) => {
                    warn!("{}", error);
                    continue;
                }
            };
            for (name, content) in map.iter() {
                trace!("Saving:{}", name);
                let file_path = data_path.clone().join(name);
                {
                    let parent_path = file_path
                        .parent()
                        .ok_or(Error::new("Failed to get parent directory."))?;
                    fs::DirBuilder::new()
                        .recursive(true)
                        .create(parent_path)
                        .map_err(|error| {
                            Error::new("An error occurred while creating the parent directory.")
                                .with_inner_error(&error)
                        })?;
                }

                let mut file = fs::File::create(file_path).map_err(|error| {
                    Error::new("An error occurred while creating the file.")
                        .with_inner_error(&error)
                })?;
                file.write_all(&mut content.clone().into_bytes())
                    .map_err(|err| {
                        Error::new("An error occurred while save the file.").with_inner_error(&err)
                    })?;
            }
        }
        let theme_path = self.get_theme_path()?;
        let theme_path = Path::new(&theme_path);
        copy_all_file(&theme_path, &data_path, |source, target| {
            trace!("Copying file from {:?} to {:?}", source, target);
            return !source.starts_with("layout");
        })?;
        return Ok(());
    }

    pub fn publish(&self) -> Result<()> {
        println!(
            "{0:>12} {1} {2}",
            "Publishing".green().bold(),
            "site",
            self.root
        );
        let build_path = self.get_build_path()?;
        let build_path = Path::new(&build_path);
        let publish_path = self.get_publish_path()?;
        let publish_path = Path::new(&publish_path);
        copy_all_file(&build_path, &publish_path, |source, target| {
            trace!("Copying file from {:?} to {:?}", source, target);
            return true;
        })?;
        return Ok(());
    }

    pub fn server(&self, port: u64) -> Result<()> {
        let mut mount = Mount::new();
        mount.mount("/", Static::new(Path::new(&self.root).join("build")));
        let address = format!("127.0.0.1:{}", port);
        println!("{0:>12} http://{1}", "Running".green().bold(), address);
        Iron::new(mount)
            .http(address)
            .map_err(|error| Error::new("Failed to lanuch server.").with_inner_error(&error))?;
        return Ok(());
    }

    fn create_render(&self) -> Result<Handlebars> {
        trace!("Creating render engine");
        let theme_path = self.get_theme_path()?;
        let layout_path = Path::new(&theme_path).join("layout");
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
            if let Some(extension) = path.extension() {
                if extension != "hbs" {
                    continue;
                }
            } else {
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
            trace!("Registering template:{}", name);
            render.register_template_file(name, &path).map_err(|err| {
                Error::new("Failed to register the template.").with_inner_error(&err)
            })?;
        }
        return Ok(render);
    }

    fn create_model(&self) -> Result<Value> {
        trace!("Creating render model");
        let mut contents = Content::load_all(&self)?;
        contents.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        let mut tags: Vec<ItemGroup<&Content>> = vec![];
        for content in &contents {
            for tag in &content.tags {
                let index = tags.iter().position(|x| x.name == tag.to_string());
                if let Some(index) = index {
                    tags[index].list.push(content);
                } else {
                    let mut ig = ItemGroup::new(tag);
                    ig.list.push(content);
                    tags.push(ig);
                }
            }
        }
        tags.sort_by(|a, b| b.list.len().cmp(&a.list.len()));

        return Ok(json!({
            "site":self.clone(),
            "contents":&contents,
            "tags":tags
        }));
    }
}
