use crate::config;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct MarkdownParserResult {
    pub front_matter: FrontMatter,
    pub body: String,
    pub index: Option<HashMap<String, IndexItem>>,
    pub tags_index: Option<Vec<String>>,
    pub lp_config: config::Config,
}

#[derive(Debug, Serialize, Clone)]
pub struct IndexItem {
    pub url: String,
    pub abst: Option<String>,
    pub front_matter: FrontMatter,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub template: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom: Option<HashMap<String, String>>,
}

fn parser_md(data: &str) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    Parser::new_ext(data, options)
}

fn parser_to_html(data: &str) -> String {
    let mut rstr = String::new();
    html::push_html(&mut rstr, parser_md(data));
    rstr
}

pub fn markdown(md: String, title: String, date: String) -> MarkdownParserResult {
    let mut mpr = front_matter_parser(md, title, date);
    let parser = parser_md(&mpr.body);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    mpr.body = html_output;

    mpr
}

pub fn front_matter_parser(md: String, title: String, date: String) -> MarkdownParserResult {
    let re = Regex::new(r"(?x)---\n(?P<front_matter>[^(---)]+)---\n").unwrap();
    let mut rmpr: MarkdownParserResult = MarkdownParserResult {
        front_matter: FrontMatter {
            title: title,
            date: date,
            template: None,
            tags: Some(Vec::new()),
            custom: Some(HashMap::new()),
        },
        body: String::from(&md),
        index: None,
        tags_index: None,
        lp_config: config::new(),
    };
    if let Some(cap) = re.captures(&md) {
        rmpr = MarkdownParserResult {
            front_matter: serde_yaml::from_str(&cap["front_matter"]).unwrap(),
            body: String::from(re.replace(&md, "")).replace("<!--more-->", ""), // replace the front_matter in the markdown
            ..rmpr
        };
    };
    rmpr
}

pub fn build_pagedata(name: &str, md: String, lp_config: &config::Config) -> MarkdownParserResult {
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
    let mut mdr = markdown(md, title, date);
    mdr.lp_config = lp_config.clone();
    mdr
}

pub fn build_abst(md_str: &str) -> Option<String> {
    let re = Regex::new(r"---(?P<abst>[^-]+)<!--more-->").unwrap();
    if let Some(caps) = re.captures(md_str) {
        Some(parser_to_html(&caps["abst"]))
    } else {
        None
    }
}
