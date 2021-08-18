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

pub fn is_project_dir() -> Result<bool, &'static str> {
    if PathBuf::from(crate::config::CONFIG_PATH).exists() {
        Ok(true)
    } else {
        Err("请在Linkpress项目文件夹内使用指令。")
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
    copy_info(p, to, "COPY ");
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
    pub fn build_index(&self, head_type: &str) -> Vec<markdown::IndexItem> {
        let mut rvec = vec![];
        let dir = match self.child_dirs.get(head_type) {
            Some(d) => d,
            None => panic!("index type not found."),
        };
        for (title, path) in &dir.child_files {
            if title == "index.md" {
                continue;
            }
            let url = path.to_str().unwrap();
            let url = String::from(url.replace(".md", "").replace(".", ""));
            let md = fs::read_to_string(path).unwrap();
            let mpr = markdown::front_matter_parser(md.clone(), title.clone(), String::from(""));
            push_sort_date(
                &mut rvec,
                markdown::IndexItem {
                    title: mpr.front_matter.title.clone(),
                    url: url,
                    abst: markdown::build_abst(&md),
                    front_matter: mpr.front_matter.clone(),
                },
            );
        }
        rvec
    }

    pub fn build_tags_index(&self) -> HashMap<String, Vec<markdown::IndexItem>> {
        let mut rhash = HashMap::new();
        for (dir_name, _) in &self.child_dirs {
            let index = self.build_index(dir_name);
            for index_item in index {
                if let Some(tags) = &index_item.front_matter.tags {
                    for tag in tags {
                        inset_to_hashmap(&mut rhash, &tag, index_item.clone());
                    }
                }
            }
        }
        rhash
    }
}

fn inset_to_hashmap(
    hash: &mut HashMap<String, Vec<markdown::IndexItem>>,
    tag: &str,
    index_item: markdown::IndexItem,
) {
    match hash.get_mut(tag) {
        Some(h) => {
            push_sort_date(h, index_item);
        }
        None => {
            let mut nh = vec![];
            push_sort_date(&mut nh, index_item);
            hash.insert(tag.to_string(), nh);
        }
    }
}

fn push_sort_date(rvec: &mut Vec<markdown::IndexItem>, item: markdown::IndexItem) {
    let mut t = 0;
    for i in rvec.clone() {
        if &i.front_matter.date <= &item.front_matter.date {
            rvec.insert(t, item.clone());
            return;
        }
        t += 1;
    }
    rvec.push(item);
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

use tera::Tera;

pub fn get_tera(theme_dir: &PathBuf) -> Tera {
    // init tera and load templates
    // 初始化 tera 并载入模板（ hbs todo ）
    let mut tera = Tera::default();
    // build a HashMap to save templates information
    // 建立一个 HashMap 暂存模板信息
    let mut files_map: HashMap<String, PathBuf> = HashMap::new();
    let re = regex::Regex::new(r"(?x)(?P<name>\w+)\.html\.tera").unwrap();
    for i in theme_dir.join("templates").read_dir().unwrap() {
        if let Ok(entry) = i {
            let file_name_ = entry.file_name();
            let file_name = file_name_.to_str().unwrap();
            let file_path = entry.path();
            let cap = re.captures(file_name).unwrap();
            files_map.insert(String::from(&cap["name"]), file_path);
        }
    }
    // trans HashMap to Vec to suit add function
    // 将暂存的模板信息转化为引用 Vec 传入 tera
    let mut files: Vec<(&std::path::Path, Option<&str>)> = vec![];
    for (k, v) in files_map.iter() {
        files.push((std::path::Path::new(v), Some(&k)))
    }
    tera.add_template_files(files).unwrap();
    tera
}
