use std;
use std::path::Path;
use std::fs;
use std::io::{Read, Write};
use std::cmp::Ordering;

use serde_json::{self, Value};
use handlebars::Handlebars;

use super::super::model::*;
use super::super::infrastructure::*;
use super::content;

type Result<T> = std::result::Result<T, Error>;

pub fn new_site() -> Result<()> {
    return init_site(".");
}

pub fn init_site(root_path: &str) -> Result<()> {
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
    let site = Site::new(root_path)?;
    let mut config_file = fs::File::create(config_path).map_err(|err| {
        Error::new("An error occurred while creating the config file.").with_inner_error(&err)
    })?;
    let content = serde_json::to_string_pretty(&site).map_err(|err| {
        Error::new("An error occurred while converting the config content.").with_inner_error(&err)
    })?;
    config_file.write(&content.into_bytes()).map_err(|err| {
        Error::new("An error occurred while writing the config file.").with_inner_error(&err)
    })?;
    return Ok(());
}

pub fn load_site(root_path: &str) -> Result<Site> {
    let path = Path::new(root_path);
    if !path.exists() {
        return Err(Error::new("The dircetory is not exists."));
    }

    let config_path = path.join("config.json");
    if !config_path.exists() {
        return Err(Error::new("The config file is not exists."));
    }

    let mut config_file = fs::File::open(config_path)
        .map_err(|error| Error::new("Failed to open the config file.").with_inner_error(&error))?;
    let mut buffer = String::new();
    config_file
        .read_to_string(&mut buffer)
        .map_err(|error| Error::new("Failed to read the config file.").with_inner_error(&error))?;

    let site = Site::load(root_path, &buffer)?;
    return Ok(site);
}

pub fn show_site(root_path: &str) -> Result<()> {
    let site = load_site(root_path)?;
    return Ok(());
}

pub fn build_site(root_path: &str) -> Result<()> {
    let site = load_site(root_path)?;

    let model = create_model(&site)?;
    let build_path = site.get_build_path()?;
    let data_path = Path::new(&build_path);
    if data_path.exists() {
        fs::remove_dir_all(&data_path).map_err(|error| {
            Error::new("Failed to clear data directory.").with_inner_error(&error)
        })?;
    }
    let mut render = create_render(&site)?;
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
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(parent_path)
                    .map_err(|error| {
                        Error::new("An error occurred while creating the parent directory.")
                            .with_inner_error(&error)
                    })?;
            }

            let mut file = fs::File::create(file_path).map_err(|error| {
                Error::new("An error occurred while creating the file.").with_inner_error(&error)
            })?;
            file.write_all(&mut content.clone().into_bytes())
                .map_err(|err| {
                    Error::new("An error occurred while save the file.").with_inner_error(&err)
                })?;
        }
    }
    let theme_path = site.get_theme_path()?;
    let theme_path = Path::new(&theme_path);
    copy_all_file(&theme_path, &data_path, |path| {
        return !path.starts_with("layout");
    })?;
    return Ok(());
}

pub fn publish_site(root_path: &str) -> Result<()> {
    let site = load_site(root_path)?;
    let build_path = site.get_build_path()?;
    let build_path = Path::new(&build_path);
    let publish_path = site.get_publish_path()?;
    let publish_path = Path::new(&publish_path);
    copy_all_file(&build_path, &publish_path, |_| {
        return true;
    })?;
    return Ok(());
}

fn create_render(site: &Site) -> Result<Handlebars> {
    let theme_path = site.get_theme_path()?;
    let layout_path = Path::new(&theme_path).join("layout");
    let templates = get_all_file(&layout_path)
        .map_err(|err| Error::new("Failed to find template files.").with_inner_error(&err))?;

    let mut render = Handlebars::new();
    render.register_helper("json", Box::new(json_helper));
    render.register_helper("file", Box::new(file_helper));
    render.register_helper("pagination", Box::new(pagination_helper));
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
            .map_err(|err| Error::new("Failed to get the template name.").with_inner_error(&err))?
            .to_str()
            .ok_or(Error::new("Format of \"path\" is incorrect ."))?;
        render
            .register_template_file(name, &path)
            .map_err(|err| Error::new("Failed to register the template.").with_inner_error(&err))?;
    }
    return Ok(render);
}

fn create_model(site: &Site) -> Result<Value> {
    let mut contents = content::load_all(site)?;
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
    let mut archives: Vec<ItemGroup<&Content>> = vec![];
    for content in &contents {
        let date = content
            .create_time
            .naive_local()
            .format("%Y-%m-%d")
            .to_string();
        let index = archives.iter().position(|x| x.name == date);
        if let Some(index) = index {
            archives[index].list.push(content);
        } else {
            let mut ig = ItemGroup::new(&date);
            ig.list.push(content);
            archives.push(ig);
        }
    }
    archives.sort_by(|a, b| b.name.cmp(&a.name));
    return Ok(json!({
            "site":site,
            "contents":&contents,
            "tags":tags,
            "archives":archives
        }));
}
