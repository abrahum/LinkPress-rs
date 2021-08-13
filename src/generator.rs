use crate::config;
use crate::logger::copy_info;
use crate::markdown;
use crate::utils;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub fn generator() {
    // load LinkPress config and cwd and more variables
    // 载入 LinkPress 的配置项，获得 cwd 和其他必要变量
    let cwd = PathBuf::from(".");
    let d = utils::build_dir(&cwd);
    let lp_config = config::load_config();

    // init target dir clear all files
    // 初始化 target 文件夹，清空现有文件
    let target_dir = cwd.join("target");
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).unwrap();
    }
    fs::create_dir(&target_dir).unwrap();

    // theme dir and config
    // 当前主题文件夹和主题配置
    let theme_dir = cwd.join("themes").join(&lp_config.site.theme);

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
    let mut files: Vec<(&Path, Option<&str>)> = vec![];
    for (k, v) in files_map.iter() {
        files.push((Path::new(v), Some(&k)))
    }
    tera.add_template_files(files).unwrap();

    // copy all theme files(except templates dir) to target dir
    // 将主题文件夹下除了 templates 的其他文件（夹）拷贝到 target 目录
    utils::cp_all_dir(&theme_dir, &target_dir);

    // copy all cwd files(only files dir will not) to target dir
    // 将根目录下的文件（不包括文件夹）拷贝至 target 目录
    copy_or_trans_dir(&d, &target_dir, &tera, &d, &lp_config);

    // tags
    let tags_dir = target_dir.join("tags");
    fs::create_dir(&tags_dir).unwrap();
    let mut context = markdown::build_pagedata("index", String::new(), &lp_config);
    let tags = utils::build_tag_vec(&d);
    context.tags_index = tags.clone();
    let contents = tera
        .render("tags", &Context::from_serialize(&context).unwrap())
        .unwrap();

    fs::write(&tags_dir.join("index.html"), contents).unwrap();
}

fn copy_or_trans_dir(
    d: &utils::Dir,
    target_dir: &PathBuf,
    tera: &Tera,
    dir_tree: &utils::Dir,
    lp_config: &config::Config,
) {
    for (file_name, file_pathbuf) in d.child_files.iter() {
        copy_or_trans_file(
            file_pathbuf,
            &target_dir.join(file_name),
            &tera,
            dir_tree,
            &lp_config,
        )
        .unwrap();
    }

    for (dir_name, dir_pathbuf) in d.child_dirs.iter() {
        let ntd = target_dir.join(dir_name);
        fs::create_dir(&ntd).unwrap();
        copy_or_trans_dir(dir_pathbuf, &ntd, tera, dir_tree, &lp_config)
    }
}

fn copy_or_trans_file(
    p: &PathBuf,
    q: &PathBuf,
    tera: &Tera,
    dir_tree: &utils::Dir,
    lp_config: &config::Config,
) -> std::io::Result<u64> {
    let file_ext = p.extension();
    if file_ext != Some(std::ffi::OsStr::new("md")) {
        copy_info(p, q, false);
        fs::copy(p, q)
    } else {
        copy_info(p, q, true);
        let mut p_ = p.clone();
        p_.pop();
        let mut f_dir_name = ".";
        // 从父目录名获得模板类型
        let mut type_ = match p_.file_name() {
            Some(osstr) => {
                f_dir_name = osstr.to_str().unwrap();
                f_dir_name
            }
            None => "index", // 根目录首页默认模板 index
        };
        let contexts: String;
        let file_name = p.file_name().unwrap();
        let md_string = fs::read_to_string(p).unwrap();
        let mut context =
            markdown::build_pagedata(file_name.to_str().unwrap(), md_string, &lp_config);
        if let Some(t) = &context.front_matter.template {
            // 从front_matter获取模板类型
            type_ = &t
        } else if file_name == "index.md" {
            // 子目录首页默认模板 index（后续考虑可客制）
            type_ = "index"
        }
        if type_ == "index" {
            let index_type: &str;
            if f_dir_name == "." {
                index_type = "posts"
            } else {
                index_type = f_dir_name
            }
            context.index = Some(dir_tree.build_index(index_type));
            contexts = tera
                .render(type_, &Context::from_serialize(&context).unwrap())
                .unwrap();
        } else {
            contexts = tera
                .render(type_, &Context::from_serialize(&context).unwrap())
                .unwrap();
        }
        let mut nq = q.clone();
        nq.pop();
        let file_stem = Path::new(file_name).file_stem().unwrap();
        nq.push(format!("{}.html", file_stem.to_str().unwrap()));
        match fs::write(nq, contexts) {
            Ok(()) => std::io::Result::Ok(1),
            Err(e) => std::io::Result::Err(e),
        }
    }
}
