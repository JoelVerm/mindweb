use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TAGS_BRACKETS_REGEX: Regex = Regex::new(r"\[\[(?P<tag>.+)\]\]").unwrap();
    static ref TAGS_HASH_REGEX: Regex = Regex::new(r"#(?P<tag>\w+)").unwrap();
    static ref TAGS_BOLD_REGEX: Regex = Regex::new(r"__+(?P<tag>.+?)__+").unwrap();
    static ref TAGS_PHONE_REGEX: Regex =
        Regex::new(r"(?P<tag>(?:0|\+\d\d ?|0?0?\d\d ?|)([1-9] ?(?:\d ?){8}))").unwrap();
    static ref TAGS_EMAIL_REGEX: Regex =
        Regex::new(r"(?P<tag>([\w\-_.]*[^.])(@\w+)(\.\w+(\.\w+)?[^.\W]))").unwrap();
}

pub fn tag_parser(contents: String) -> Vec<String> {
    let run_regex = |regex: &Regex| -> Vec<String> {
        regex
            .captures_iter(&contents)
            .map(|element| String::from(&(element["tag"])))
            .collect()
    };
    let mut tags = vec![];
    tags.extend(run_regex(&TAGS_BRACKETS_REGEX));
    tags.extend(run_regex(&TAGS_HASH_REGEX));
    tags.extend(run_regex(&TAGS_BOLD_REGEX));
    tags.extend(run_regex(&TAGS_PHONE_REGEX));
    tags.extend(run_regex(&TAGS_EMAIL_REGEX));
    return tags;
}
