use dirs::home_dir;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::String;

pub mod macos;
pub mod unsplash;

/// download a wallpaper, save to a file.
pub fn download_wallpaper(save_path: PathBuf) -> PathBuf {
    let mut f = save_path;
    f.push("desktop.jpg");

    let save_path: String = String::from_str(f.to_str().unwrap()).unwrap();
    let ret_save_path = save_path.clone();
    unsplash::get_random_image(save_path);
    PathBuf::from_str(&ret_save_path[..]).unwrap()
}

/// set wallpaper
pub fn set_wallpaper(wallpaper_path: PathBuf) -> PathBuf {
    macos::set_wallpaper(wallpaper_path.to_str().unwrap());
    wallpaper_path
}

/// get unsplashme temp file path
pub fn create_temp_path() -> PathBuf {
    let mut dir = home_dir().unwrap_or_default();
    dir.push(Path::new(".unsplashme"));
    let r = dir.to_owned();
    create_dir_all(dir).unwrap();
    r
}
