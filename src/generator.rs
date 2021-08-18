use crate::config;
use crate::logger::copy_info;
use crate::markdown;
use crate::utils;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub fn generator() {
    match utils::is_project_dir() {
        Ok(_) => {
            generate();
        }
        Err(s) => println!("{}", s),
    }
}

fn generate() {
    // load LinkPress config and cwd and more variables
    // 载入 LinkPress 的配置项，获得 cwd 和其他必要变量
    let cwd = PathBuf::from(".");
    let d = utils::build_dir(&cwd);
    let lp_config = config::load_config();

    // init target dir clear all files
    // 初始化 target 文件夹，清空现有文件
    let target_dir = cwd.join("target");
    if target_dir.exists() {
        clear_target_dir(&target_dir)
    } else {
        fs::create_dir(&target_dir).unwrap();
    }

    // theme dir and config
    // 当前主题文件夹和主题配置
    let theme_dir = cwd.join("themes").join(&lp_config.site.theme);

    // init tera and load templates
    // 初始化 tera 并载入模板（ hbs todo ）
    let tera = utils::get_tera(&theme_dir);

    // copy all theme files(except templates dir) to target dir
    // 将主题文件夹下除了 templates 的其他文件（夹）拷贝到 target 目录
    utils::cp_all_dir(&theme_dir, &target_dir);

    // copy all cwd files(only files dir will not) to target dir
    // 将根目录下的文件（不包括文件夹）拷贝至 target 目录
    copy_or_trans_dir(&d, &target_dir, &tera, &d, &lp_config);

    // tags
    let tags_dir = target_dir.join("tags");
    copy_info(&PathBuf::from("tags"), &tags_dir, "BUILD");
    fs::create_dir(&tags_dir).unwrap();
    let mut context = markdown::build_pagedata("index", String::new(), &lp_config);
    let tags = utils::build_tag_vec(&d);
    context.tags_index = tags.clone();
    let contents = tera
        .render("tags", &Context::from_serialize(&context).unwrap())
        .unwrap();
    fs::write(&tags_dir.join("index.html"), contents).unwrap();

    let tags_index = d.build_tags_index();
    for tag in tags.unwrap() {
        context.index = Some(tags_index.get(&tag).unwrap().clone());
        let contents = tera
            .render("index", &Context::from_serialize(&context).unwrap())
            .unwrap();
        fs::write(&tags_dir.join(format!("{}.html", &tag)), contents).unwrap();
    }
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
        copy_info(p, q, "COPY ");
        fs::copy(p, q)
    } else {
        copy_info(p, q, "TRANS");
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

fn clear_target_dir(target_dir: &PathBuf) {
    for i in target_dir.read_dir().unwrap() {
        if let Ok(entry) = i {
            let ep = entry.path();
            if ep.is_file() {
                fs::remove_file(ep).unwrap();
            } else if ep.is_dir() {
                if ep.file_name().unwrap() != ".git" {
                    fs::remove_dir_all(ep).unwrap()
                }
            }
        }
    }
}
