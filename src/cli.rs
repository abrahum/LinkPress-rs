use crate::config;
use crate::markdown;
use chrono::Local;
use serde_yaml;
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

const DEFAULE_DIR_NAME: &str = "Linkpress-rs";
const DEFAULT_TYPE: &str = "post";

pub fn init(dir_name_input: Option<&str>) {
    let dir_name: &str;
    if let Some(x) = dir_name_input {
        dir_name = x;
    } else {
        dir_name = DEFAULE_DIR_NAME;
    }

    if had(dir_name) {
        println!("当前目录下已存在{}，无需再次初始化。", dir_name);
        std::process::exit(101);
    } else {
        fs::create_dir(dir_name).unwrap();
        config::new().save(Some(dir_name));
        let path = PathBuf::from(dir_name);
        fs::create_dir(path.join("themes")).unwrap();
        fs::create_dir(path.join("posts")).unwrap();
        fs::write(
            path.join("index.md"),
            generate_front_matter("index", Some("index")),
        )
        .unwrap();
    }
}

pub fn new(type_: Option<&str>, name: &str) {
    match crate::utils::is_project_dir() {
        Ok(_) => _new(type_, name),
        Err(s) => println!("{}", s),
    }
}

fn _new(type_: Option<&str>, name: &str) {
    let dir_name: String;
    if let Some(x) = type_ {
        dir_name = format!("{}s", x);
    } else {
        dir_name = format!("{}s", DEFAULT_TYPE);
    }

    if !had(&dir_name) {
        fs::create_dir(&dir_name).unwrap();
    }

    let today = Local::now();
    let today = today.format("%Y-%m-%d");
    let file_path = PathBuf::from(&dir_name).join(format!("{}-{}.md", today, name));
    fs::write(file_path, "").unwrap();
}

fn had(name: &str) -> bool {
    let cwd = current_dir().unwrap();
    let mut had = false;
    for file in fs::read_dir(cwd).unwrap() {
        let file = file.unwrap().path();
        if let Some(file_name) = file.file_name() {
            if file_name == name {
                had = true;
                break;
            }
        }
    }
    had
}

fn generate_front_matter(title: &str, template: Option<&str>) -> String {
    let today = Local::now();
    let today = today.format("%Y-%m-%d");
    let front_matter = markdown::FrontMatter {
        title: title.to_string(),
        date: today.to_string(),
        template: match template {
            Some(s) => Some(s.to_string()),
            None => None,
        },
        tags: None,
        custom: None,
    };
    let yaml = match serde_yaml::to_string(&front_matter) {
        Ok(r) => format!("{}---", r),
        Err(e) => panic!("{}", e),
    };
    yaml
}
