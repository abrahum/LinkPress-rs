use std::path::PathBuf;

pub fn deployer() {
    match crate::utils::is_project_dir() {
        Ok(_) => {
            deploy();
        }
        Err(s) => println!("{}", s),
    }
}

fn had_git(p: &PathBuf) -> bool {
    let mut had = false;
    for f in p.read_dir().unwrap() {
        if let Ok(entry) = f {
            if entry.path().file_name().unwrap().to_str().unwrap() == ".git" {
                had = true;
                break;
            }
        }
    }
    had
}

fn deploy() {
    let cwd = PathBuf::from(".");
    let target_dir = cwd.join("target");
    if !target_dir.exists() {
        crate::logger::warn("Target dir not exists, please generate first.");
        std::process::exit(101);
    }
    if !had_git(&target_dir) {
        // todo
    }
}
