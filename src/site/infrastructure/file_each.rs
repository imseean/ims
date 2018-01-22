use std::fs::*;
use std::path::Path;

pub fn get_all_file(path: &Path) -> Vec<String> {
    println!("{:?}", path);
    let mut list: Vec<String> = vec![];
    if !path.exists() {
        return vec![];
    } else if path.is_dir() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let sub_path = entry.path();
            let mut sub_list = get_all_file(&sub_path);
            list.append(&mut sub_list);
        }
    } else {
        list.push(path.to_str().map(|s| s.to_string()).unwrap());
    }
    return list;
}
