use dirs::home_dir;
use std::path::Path;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub mod unsplash;

pub trait Wallpaper {
    type T: Config;
    /// create an instance of Wallpaper implementation.
    fn new(config: Self::T) -> Box<Self>;

    /// create temp path
    fn create_temp_path(&self) -> PathBuf {
        let mut dir = home_dir().unwrap_or_default();
        dir.push(Path::new(".unsplashme"));
        let r = dir.to_owned();
        create_dir_all(dir).unwrap();
        r
    }

    /// download picture
    fn download_wallpaper(&self, save_path: &PathBuf) -> PathBuf;

    /// set wallpaper with saved picture
    fn set_wallpaper(&self, picture_path: &PathBuf);
}

pub trait Config {}
