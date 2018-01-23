use std;
use std::fs::*;
use std::path::Path;
use super::Error;

type Result<T> = std::result::Result<T, Error>;

pub fn get_all_file(path: &Path) -> Result<Vec<String>> {
    let mut list: Vec<String> = vec![];
    if !path.exists() {
        return Ok(vec![]);
    } else if path.is_dir() {
        for entry in read_dir(path)
            .map_err(|err| Error::new("Failed to open Directory.").with_inner_error(&err))?
        {
            let entry = entry
                .map_err(|err| Error::new("Failed to open Directory.").with_inner_error(&err))?;
            let sub_path = entry.path();
            let mut sub_list = get_all_file(&sub_path)?;
            list.append(&mut sub_list);
        }
    } else {
        list.push(
            path.to_str()
                .ok_or(Error::new("Format of \"path\" is incorrect."))?
                .to_string(),
        );
    }
    return Ok(list);
}

pub fn copy_all_file<F: Fn(&Path) -> bool>(source: &Path, target: &Path, filter: F) -> Result<()> {
    if source.is_file() {
        return Err(Error::new("Source path must be a directory."));
    }
    if target.is_file() {
        return Err(Error::new("Target path must be a directory."));
    }
    if !target.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(target)
            .map_err(|error| {
                Error::new("An error occurred while creating the target directory.")
                    .with_inner_error(&error)
            })?;
    }
    let files = get_all_file(&source)?;
    for file in &files {
        let source_file_path = Path::new(file);
        let path = source_file_path
            .strip_prefix(source
                .to_str()
                .ok_or(Error::new(&format!("Format of \"source\" is incorrect.")))?)
            .map_err(|err| {
                Error::new(&format!(
                    "The Path is not The child path of the parent path."
                )).with_inner_error(&err)
            })?;
        if filter(&path) {
            let target_file_path = target.join(path);
            let parent_path = target_file_path
                .parent()
                .ok_or(Error::new("Failed to get parent directory."))?;
            DirBuilder::new()
                .recursive(true)
                .create(parent_path)
                .map_err(|error| {
                    Error::new("An error occurred while creating the parent directory.")
                        .with_inner_error(&error)
                })?;
            copy(&source_file_path, &target_file_path)
                .map_err(|error| Error::new("Failed to copy file.").with_inner_error(&error))?;
        }
    }

    return Ok(());
}
