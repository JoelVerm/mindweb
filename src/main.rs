mod files_tags;
mod tag_parser;

use crate::files_tags::get_files_tags;
use crate::tag_parser::tag_parser;

fn main() {
    let files_tags = get_files_tags(&tag_parser);
    for (file, tags) in files_tags {
        println!("{file} -> {}", tags.join(", "))
    }
}
