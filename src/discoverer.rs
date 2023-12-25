use std::{env,fs, io, path::PathBuf};

const UNIX_HOME_PATH_KEY: &str = "HOME";
const WINDOWS_HOME_PATH_KEY: &str = "USERPROFILE";

pub struct Discovered {
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>,
}

fn get_os_var(key: &str) -> Result<String, &'static str> {
    match env::var_os(key) {
        Some(os_dir) => {
            if let Some(os_dir_str) = os_dir.to_str() {
                Ok(String::from(os_dir_str))
            } else {
                Err("Error on get user path")
            }
        },
        None => Err("Error on get user OS path"),
    }
}

fn user_path_by_os() -> Result<String, &'static str> {
    match env::consts::OS {
        "linux" => get_os_var(UNIX_HOME_PATH_KEY),
        "macos" => get_os_var(UNIX_HOME_PATH_KEY),
        "windows" => get_os_var(WINDOWS_HOME_PATH_KEY),
        _ => panic!("OS not supported"),
    }
}

pub fn discover_directories() -> Result<Discovered, io::Error>{
    let user_path = user_path_by_os().unwrap();
    let mut disc: Discovered = Discovered{ files: Vec::new(), dirs: Vec::new() };

    for entry in fs::read_dir(&user_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            disc.files.push(path);
        } else {
            disc.dirs.push(path);
        }
    }

    Ok(disc)
}