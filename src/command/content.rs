use std;
use std::io;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

use prettytable::{format, Table};
use serde_json;

use super::super::model::*;
use super::super::infrastructure::*;

type Result<T> = std::result::Result<T, Error>;

pub fn new_content(site: &Site, path: &str) -> Result<Content> {
    let content_path = site.get_content_path()?;
    let file_path = Path::new(&content_path).join(&path);

    if file_path.exists() {
        print!(
            "The file ({}) already exists. Type [Y] to overwrite it .[Y/N]",
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
            return Err(Error::new("The file already exists."));
        }
    }
    let parent_path = file_path.parent().ok_or(Error::new(
        "An error occurred while getting the parent directory from the path.",
    ))?;
    if !parent_path.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(|err| {
                Error::new("An error occurred while creating the parent directory.")
                    .with_inner_error(&err)
            })?;
    }

    let mut file = fs::File::create(&file_path).map_err(|err| {
        Error::new("An error occurred while creating the file.").with_inner_error(&err)
    })?;
    let content = Content::new(&site, &content_path)?;

    let mark = serde_json::to_string_pretty(&content)
        .map_err(|err| Error::new("An error occurred while save the file.").with_inner_error(&err))?
        .to_string();
    let data = format!("``````` json\r\n{}\r\n```````\r\n{}", mark, content.content);
    file.write_all(&mut data.into_bytes()).map_err(|err| {
        Error::new("An error occurred while save the file.").with_inner_error(&err)
    })?;
    return Ok(content);
}

pub fn load(site: &Site, path: &str) -> Result<Content> {
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

    let content = Content::load(&site, &path, &buffer)?;

    return Ok(content);
}

pub fn list_content(site: &Site) -> Result<()> {
    let list: Vec<Content> = load_all(site)?;
    let mut table = Table::new();
    table.set_titles(row![
        "TITLE",
        "CREATE DATE",
        "TAGS",
        "PATH",
        "ID"
    ]);
    for item in &list {
        table.add_row(row![
            item.title,
            item.create_time.naive_local().format("%Y-%m-%d %H:%M:%S"),
            item.tags.join("/"),
            item.path,
            item.id
        ]);
    }
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.printstd();
    return Ok(());
}

pub fn load_all(site: &Site) -> Result<Vec<Content>> {
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
        let content = match load(&site, &path) {
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
