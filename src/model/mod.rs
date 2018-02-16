mod content;
mod site;

pub use self::site::Site;
pub use self::content::Content;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemGroup<T>{
    pub name:String,
    pub list:Vec<T>
}
impl<T> ItemGroup<T>{
    pub fn new(name:&str)->ItemGroup<T>{
        return ItemGroup{
            name:name.to_string(),
            list:vec![]
        };
    }
}
