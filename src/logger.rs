use colored::Colorize;
use log::{info, warn};
use std::path::PathBuf;

const LINKPRESS: &'static str = "LinkPress";

pub fn info(message: String) {
    info!(target: LINKPRESS, "{}", message);
}

pub fn copy_info(from: &PathBuf, to: &PathBuf, type_: &str) {
    info(format!(
        "{}: {} to {}",
        type_.bright_cyan(),
        from.to_str().unwrap().green(),
        to.to_str().unwrap().replace(".md", ".html").green()
    ));
}

pub fn warn(message: &str) {
    warn!(target: LINKPRESS, "{}", message);
}
