mod content;
mod infrastructure;
use self::content::Content;
use serde_json;
use std::path::Path;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use prettytable::{format, Table};
use chrono::{DateTime, Local, TimeZone};

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub author: String,
    pub title: String,
    pub subtitle: String,
    pub baseurl: String,
    pub theme: String,
    pub layout_directory: String,
    pub content_directory: String,
    pub data_directory: String,
    pub build_directory: String,
    pub publish_directory: String,
    pub assets_directory: String,
    #[serde(skip)] pub root: String,
}

impl Site {
    pub fn load(root_path: &str) -> Site {
        let root = root_path.to_string();
        let path = Path::new(root_path);
        if !path.exists() {
            panic!("Dircetory is not exists.");
        }

        let config_path = path.join("config.json");
        let mut site = Site {
            name: "Ims".to_string(),
            author: "<author>".to_string(),
            title: "<title>".to_string(),
            subtitle: "<subtitle>".to_string(),
            baseurl: "<baseurl>".to_string(),
            theme: "default".to_string(),
            layout_directory: "layout".to_string(),
            content_directory: "content".to_string(),
            data_directory: "data".to_string(),
            build_directory: "build".to_string(),
            publish_directory: "publish".to_string(),
            assets_directory: "assets".to_string(),
            root: root,
        };
        if config_path.exists() {
            let mut config_file = File::open(config_path).unwrap();
            let mut buffer = String::new();
            config_file.read_to_string(&mut buffer).unwrap();
            let value = serde_json::to_value(buffer).unwrap();
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
            site.layout_directory = value["layout_directory"]
                .as_str()
                .map(|x| x.to_string())
                .unwrap_or(site.layout_directory);
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
        }
        Content::load(&site, "./hello.md");
        return site;
    }

    pub fn new(root_path: &str) -> Site {
        let root = root_path.to_string();
        let path = Path::new(root_path);
        if !path.exists() {
            DirBuilder::new().recursive(true).create(path).unwrap();
        }
        let config_path = path.join("config.json");
        if config_path.exists() && config_path.is_file() {
            panic!("Config file is exists.");
        }

        let mut site = Site {
            name: "Ims".to_string(),
            author: "<author>".to_string(),
            title: "<title>".to_string(),
            subtitle: "<subtitle>".to_string(),
            baseurl: "<baseurl>".to_string(),
            theme: "default".to_string(),
            layout_directory: "layout".to_string(),
            content_directory: "content".to_string(),
            data_directory: "data".to_string(),
            build_directory: "build".to_string(),
            publish_directory: "publish".to_string(),
            assets_directory: "assets".to_string(),
            root: root,
        };
        let mut config_file = File::create(config_path).unwrap();
        let content = serde_json::to_string_pretty(&site).unwrap_or("{}".to_string());
        config_file.write(&content.into_bytes()).unwrap();

        return site;
    }

    pub fn new_content(&self, content_path: &str) {
        let content = Content::new(self, content_path);
        Site::show_content_info(&content);
    }
    pub fn generate(&self) {}
    pub fn publish(&self) {}
    pub fn server(&self) {}

    pub fn list_content(&self) {
        let paths = Content::find_all_content(self);
        let list: Vec<Content> = paths.iter().map(|x| Content::load(self, x)).collect();
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
        println!("TOTAL:{}", list.len());
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
