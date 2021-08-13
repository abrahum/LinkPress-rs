use crate::logger::copy_info;
use crate::markdown;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize)]
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

pub fn build_dir(p: &PathBuf) -> Dir {
    let mut rdit = Dir {
        child_files: HashMap::new(),
        child_dirs: HashMap::new(),
    };
    for i in p.read_dir().unwrap() {
        if let Ok(entry) = i {
            let ep = entry.path();
            if ep.is_dir() && !in_black(&ep, true) {
                let child_dirs = build_dir(&ep);
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

impl Dir {
    pub fn build_index(&self, head_type: &str) -> HashMap<String, markdown::IndexItem> {
        let mut rhash: HashMap<String, markdown::IndexItem> = HashMap::new();
        let dir = match self.child_dirs.get(head_type) {
            Some(d) => d,
            None => panic!("index type not found."),
        };
        for (title, path) in &dir.child_files {
            let url = path.to_str().unwrap();
            let url = String::from(url.replace(".md", "").replace(".", ""));
            let md = fs::read_to_string(path).unwrap();
            let mpr = markdown::front_matter_parser(md.clone(), title.clone(), String::from(""));
            rhash.insert(
                mpr.front_matter.title.clone(),
                markdown::IndexItem {
                    url: url,
                    abst: markdown::build_abst(&md),
                    front_matter: mpr.front_matter.clone(),
                },
            );
        }
        rhash
    }

    pub fn build_tags_index(&self) -> HashMap<String, HashMap<String, markdown::IndexItem>> {
        let mut rhash = HashMap::new();
        for (dir_name, _) in &self.child_dirs {
            let index = self.build_index(dir_name);
            for (page_name, index_item) in index {
                if let Some(tags) = &index_item.front_matter.tags {
                    for tag in tags {
                        inset_to_hashmap(
                            &mut rhash,
                            &tag,
                            page_name.to_string(),
                            index_item.clone(),
                        );
                    }
                }
            }
        }
        rhash
    }
}

fn inset_to_hashmap(
    hash: &mut HashMap<String, HashMap<String, markdown::IndexItem>>,
    tag: &str,
    page_name: String,
    index_item: markdown::IndexItem,
) {
    match hash.get_mut(tag) {
        Some(h) => {
            h.insert(page_name, index_item);
        }
        None => {
            let mut nh = HashMap::new();
            nh.insert(page_name, index_item);
            hash.insert(tag.to_string(), nh);
        }
    }
}

pub fn build_tag_vec(d: &Dir) -> Option<Vec<String>> {
    let mut tags = vec![];
    for tag in d.build_tags_index().keys() {
        tags.push(tag.clone());
    }
    if tags.is_empty() {
        None
    } else {
        Some(tags)
    }
}
