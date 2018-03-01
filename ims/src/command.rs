use std;

use super::model::*;
use super::infrastructure::*;
type Result<T> = std::result::Result<T, Error>;

pub mod site_command {
    use super::*;
    pub fn init() -> Result<()> {
        new(".")?;
        return Ok(());
    }

    pub fn new(root_path: &str) -> Result<()> {
        Site::new(root_path)?;
        return Ok(());
    }

    pub fn info(root_path: &str) -> Result<()> {
        let site = Site::load(root_path)?;
        site.info()?;
        return Ok(());
    }

    pub fn build(root_path: &str) -> Result<()> {
        let site = Site::load(root_path)?;
        site.build()?;
        return Ok(());
    }

    pub fn publish(root_path: &str) -> Result<()> {
        let site = Site::load(root_path)?;

        site.publish()?;
        return Ok(());
    }
    pub fn server(root_path: &str, port: u64) -> Result<()> {
        let site = Site::load(root_path)?;
        site.server(port)?;
        return Ok(());
    }
}

pub mod content_command {
    use super::*;
    pub fn new(root_path: &str, path: &str) -> Result<()> {
        let site = Site::load(root_path)?;
        Content::new(&site, path)?;
        return Ok(());
    }

    pub fn list(root_path: &str) -> Result<()> {
        let site = Site::load(root_path)?;
        Content::list(&site)?;
        return Ok(());
    }
}
