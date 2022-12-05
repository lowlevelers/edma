use path_absolutize::*;
use std::{env, path::Path};

pub fn get_absolute_path(path: &str) -> String {
	let p = Path::new(path);
	let cwd = env::current_dir().unwrap();

	p.absolutize_from(&cwd).unwrap().to_str().unwrap().to_string()
}
