use std::env;

const UNIX_HOME_PATH_KEY: &str = "HOME";
const WINDOWS_HOME_PATH_KEY: &str = "USERPROFILE";

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

pub fn user_path_by_os() -> Result<String, &'static str> {
    match env::consts::OS {
        "linux" => get_os_var(UNIX_HOME_PATH_KEY),
        "macos" => get_os_var(UNIX_HOME_PATH_KEY),
        "windows" => get_os_var(WINDOWS_HOME_PATH_KEY),
        _ => panic!("OS not supported"),
    }
}
