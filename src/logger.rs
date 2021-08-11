use log::info;
use std::env;
use std::path::PathBuf;

pub fn info(message: String) {
    info!(target:"LinkPress", "{}",message);
}

pub fn copy_info(from: &PathBuf, to: &PathBuf, trans: bool) {
    let cwd = env::current_dir().unwrap();
    let from_s = from.to_str().unwrap().replace(cwd.to_str().unwrap(), "");
    let to_s = to.to_str().unwrap().replace(cwd.to_str().unwrap(), "");
    info(format!(
        "{}: {} to {}",
        if trans { "Transfroming" } else { "Coping" },
        from_s,
        if trans {
            to_s.replace(".md", ".html")
        } else {
            to_s
        }
    ));
}
