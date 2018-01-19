#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryMap {
    pub layout: String,
    pub content: String,
    pub data: String,
    pub build: String,
    pub publish: String,
    pub assets: String,
}

impl DirectoryMap {
    pub fn new() -> DirectoryMap {
        return DirectoryMap {
            layout: "layout".to_string(),
            content: "content".to_string(),
            data: "data".to_string(),
            build: "build".to_string(),
            publish: "publish".to_string(),
            assets: "assets".to_string(),
        };
    }
}
