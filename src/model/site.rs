use std;
use std::path::Path;

use serde_json;

pub use super::content::Content;
use super::super::infrastructure::*;

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
        let path = Path::new(&self.root)
            .join(&self.theme_directory)
            .join(&self.theme);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get data path."))?
            .to_string();
        return Ok(path);
    }
    pub fn get_build_path(&self) -> Result<String> {
        let path = Path::new(&self.root)
            .join(&self.theme_directory)
            .join(&self.theme);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get build path."))?
            .to_string();
        return Ok(path);
    }
    pub fn get_publish_path(&self) -> Result<String> {
        let path = Path::new(&self.root)
            .join(&self.theme_directory)
            .join(&self.theme);
        let path = path.to_str()
            .ok_or(Error::new("Failed to get publish path."))?
            .to_string();
        return Ok(path);
    }

    /// Load site config from the specified directory.
    pub fn load(root_path: &str, data: &str) -> Result<Site> {
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
            root: root_path.to_string(),
        };
        let value = serde_json::to_value(data).map_err(|error| {
            Error::new("Failed to resolve the config file.").with_inner_error(&error)
        })?;
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
        let site = Site {
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
            root: root_path.to_string(),
        };
        return Ok(site);
    }
}
