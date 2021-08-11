use crate::utils::Dir;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct MarkdownParserResult<'a> {
    pub front_matter: FrontMatterItem,
    pub body: String,
    dir_tree: &'a Dir,
}

pub fn markdown(md: String, title: String, date: String, dir_tree: &Dir) -> MarkdownParserResult {
    let mut mpr = front_matter_parser(md, title, date, dir_tree);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&mpr.body, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    mpr.body = html_output;

    mpr
}

fn front_matter_parser(
    md: String,
    title: String,
    date: String,
    dir_tree: &Dir,
) -> MarkdownParserResult {
    let re = Regex::new(r"(?x)---(?P<front_matter>[^(---)]+)---").unwrap();
    let mut rmpr: MarkdownParserResult = MarkdownParserResult {
        front_matter: FrontMatterItem {
            title: title,
            date: date,
            template: None,
            tags: Some(Vec::new()),
            custom: Some(HashMap::new()),
        },
        body: String::from(&md),
        dir_tree: dir_tree,
    };
    if let Some(cap) = re.captures(&md) {
        rmpr = MarkdownParserResult {
            front_matter: serde_yaml::from_str(&cap["front_matter"]).unwrap(),
            body: String::from(re.replace(&md, "")), // replace the front_matter in the markdown
            ..rmpr
        };
    };
    rmpr
}

#[derive(Serialize, Debug, Deserialize)]
pub struct FrontMatterItem {
    title: String,
    date: String,
    pub template: Option<String>,
    tags: Option<Vec<String>>,
    custom: Option<HashMap<String, String>>,
}

pub fn build_pagedata<'a>(name: &str, md: String, dir_tree: &'a Dir) -> MarkdownParserResult<'a> {
    let date: String;
    let title: String;
    let re = Regex::new(
        r"(?x)
    (?P<date>\d{4}-\d{2}-\d{2})
    -(?P<title>[^\.]+)",
    )
    .unwrap();
    match re.captures(name) {
        Some(cap) => {
            date = String::from(&cap["date"]);
            title = String::from(&cap["title"]);
        }
        None => {
            date = String::from("-");
            title = String::from("File name illegal");
        }
    };
    let mdr = markdown(md, title, date, dir_tree);
    mdr
}
