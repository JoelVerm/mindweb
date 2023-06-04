mod files_tags;
mod structs;
mod tag_parser;

use std::collections::HashMap;

use crate::files_tags::get_files_tags;
use crate::structs::{Item, Tag};

fn hashmap_merge_from_ref(map: &mut HashMap<Item, Vec<Tag>>, map_ref: &HashMap<Item, Vec<Tag>>) {
    map.extend(map_ref.into_iter().map(|(k, v)| (k.clone(), v.clone())));
}

fn main() {
    let mut tags: HashMap<Item, Vec<Tag>> = HashMap::new();

    let files_tags = get_files_tags();
    hashmap_merge_from_ref(&mut tags, &files_tags);

    for (file, tags) in tags {
        println!(
            "{} -> {}",
            file.location,
            tags.iter()
                .map(|tag| tag.text.clone())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
