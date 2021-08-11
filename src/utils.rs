use crate::logger::copy_info;
use crate::markdown;
use regex::Regex;
use std::collections::HashMap;
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

#[derive(Debug, Clone)]
pub struct Dir {
    pub child_files: HashMap<String, PathBuf>,
    pub child_dirs: HashMap<String, Dir>,
}

const DIR_BALAK_ARRAY: [&'static str; 3] = ["themes", "target", "templates"];
const FILE_BLACK_ARRAY: [&'static str; 3] = ["LinkPress.toml", "theme.toml", "Cargo.toml"];

fn in_black(p: &PathBuf, is_dir: bool) -> bool {
    let file_name = p.file_name().unwrap();
    let mut r = false;
    let black_array = if is_dir {
        DIR_BALAK_ARRAY
    } else {
        FILE_BLACK_ARRAY
    };
    for black in black_array.iter() {
        if &file_name.to_str().unwrap() == black {
            r = true;
            break;
        }
    }
    r
}

pub fn build_map(p: &Path) -> Dir {
    let mut rdit = Dir {
        child_files: HashMap::new(),
        child_dirs: HashMap::new(),
    };
    for i in p.read_dir().unwrap() {
        if let Ok(entry) = i {
            let ep = entry.path();
            if ep.is_dir() && !in_black(&ep, true) {
                let child_dirs = build_map(ep.as_path());
                rdit.child_dirs.insert(
                    String::from(ep.file_name().unwrap().to_str().unwrap()),
                    child_dirs,
                );
            } else if ep.as_path().is_file() && !in_black(&ep, false) {
                rdit.child_files
                    .insert(String::from(ep.file_name().unwrap().to_str().unwrap()), ep);
            }
        }
    }
    rdit
}

pub fn cp_all_dir(p: &PathBuf, to: &PathBuf) {
    copy_info(p, to, false);
    for i in p.read_dir().unwrap() {
        if let Ok(entry) = i {
            let ep = entry.path();
            if ep.as_path().is_file() && !in_black(&ep, false) {
                fs::copy(&ep, to.join(&ep.file_name().unwrap())).unwrap();
            } else if ep.as_path().is_dir() && !in_black(&ep, true) {
                let next_to = to.join(&ep.file_name().unwrap());
                fs::create_dir(next_to.as_path()).unwrap();
                cp_all_dir(&ep, &next_to);
            }
        }
    }
}
