use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process;

pub fn get_files_tags(tag_parser: &dyn Fn(String) -> Vec<String>) -> HashMap<String, Vec<String>> {
    let mut tags_map: HashMap<String, Vec<String>> = HashMap::new();
    let basedir_env = format!("{}/mindweb/", env::var("HOME").unwrap());
    let basedir_path = Path::new(&basedir_env);
    let basedir_entries = fs::read_dir(&basedir_path).unwrap_or_else(|_err| {
        println!("Please create the folder ~/mindweb/ first");
        process::exit(1);
    });
    for dir_entry in basedir_entries {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();
        let path_string = String::from(path.to_str().unwrap());
        let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        if !["", "txt", "md"].contains(&extension) {
            continue;
        }
        let contents = match fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => continue,
        };
        let tags = tag_parser(contents);
        tags_map.insert(format!("file:{path_string}"), tags);
    }
    return tags_map;
}
