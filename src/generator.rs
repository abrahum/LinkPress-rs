use crate::utils;
use std::env;
use std::path::Path;

pub fn generator() {
    let cwd = env::current_dir().unwrap();
    let d = utils::build_map(Path::new(&cwd));
    println!("{:#?}", d);
}
