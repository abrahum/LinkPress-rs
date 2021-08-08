use crate::markdown;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_type_path(type_: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(type_);
    if path.exists() {
        Ok(path)
    } else {
        Err("This type of page do not exists.")
    }
}

pub fn get_page(type_: &str, name: &str) -> Result<String, &'static str> {
    match get_type_path(type_) {
        Ok(path) => {
            let path = path.join(format!("{}.md", name));
            if let Ok(f) = fs::read_to_string(path) {
                Ok(f)
            } else {
                Err("Markdown file not found.")
            }
        }
        Err(e) => Err(e),
    }
}

pub fn build_pagedata(name: &str, md: String) -> markdown::MarkdownParserResult {
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
    let mdr = markdown::markdown(md, title, date);
    mdr
}

#[derive(Debug)]
pub struct Dir {
    dir_name: String,
    child_files: Vec<String>,
    child_dirs: Vec<Dir>,
}

pub fn build_map(p: &Path) -> Dir {
    let mut rdit = Dir {
        dir_name: String::from(p.file_name().unwrap().to_str().unwrap()),
        child_files: Vec::new(),
        child_dirs: Vec::new(),
    };
    for i in p.read_dir().unwrap() {
        if let Ok(entry) = i {
            if entry.path().as_path().is_dir() {
                let cd = build_map(entry.path().as_path());
                rdit.child_dirs.push(cd);
            } else if entry.path().as_path().is_file() {
                rdit.child_files
                    .push(String::from(entry.path().as_path().to_str().unwrap()))
            }
        }
    }
    rdit
}
