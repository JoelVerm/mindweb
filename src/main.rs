use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn get_contents_tags(contents: String) -> Vec<String> {
    lazy_static! {
        static ref TAGS_BRACKETS_REGEX: Regex = Regex::new(r"\[\[(?P<tag>.*)\]\]").unwrap();
        static ref TAGS_HASH_REGEX: Regex = Regex::new(r"#(?P<tag>\w*)").unwrap();
        static ref TAGS_PHONE_REGEX: Regex =
            Regex::new(r"(?P<tag>(?:0|\+\d\d ?|0?0?\d\d ?|)([1-9] ?(?:\d ?){8}))").unwrap();
    }
    let tags_brackets: Vec<String> = TAGS_BRACKETS_REGEX
        .captures_iter(&contents)
        .map(|element| String::from(&(element["tag"])))
        .collect();
    let tags_hash: Vec<String> = TAGS_HASH_REGEX
        .captures_iter(&contents)
        .map(|element| String::from(&(element["tag"])))
        .collect();
    let tags_phone: Vec<String> = TAGS_PHONE_REGEX
        .captures_iter(&contents)
        .map(|element| String::from(&(element["tag"])))
        .collect();
    let mut tags = vec![];
    tags.extend(tags_brackets);
    tags.extend(tags_hash);
    tags.extend(tags_phone);
    println!("{}", tags.join(" "));
    return tags;
}

fn get_files_tags() -> HashMap<String, Vec<String>> {
    let mut tags_map: HashMap<String, Vec<String>> = HashMap::new();
    let basedir_env = format!("{}/mindweb/", env::var("HOME").unwrap());
    let basedir_path = Path::new(&basedir_env);
    let basedir_entries = fs::read_dir(&basedir_path).unwrap_or_else(|_err| {
        println!("Please create the folder ~/mindweb/ first");
        process::exit(1);
    });
    for dir_entry in basedir_entries {
        let path = dir_entry.unwrap().path();
        let contents = fs::read_to_string(&path).unwrap();
        let path_string = String::from(path.to_str().unwrap());
        let tags = get_contents_tags(contents);
        tags_map.insert(path_string, tags);
    }
    return tags_map;
}

fn main() {
    get_files_tags();
}
