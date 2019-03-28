use super::super::Wallpaper;
use super::config::UnsplashConfig;
use super::unsplash;
use std::path::PathBuf;
use std::process::Command;
use whoami::{env, DesktopEnv::Mac, DesktopEnv::Gnome};

#[derive(Debug)]
pub struct UnsplashWallpaper {
    config: UnsplashConfig,
}

impl Wallpaper for UnsplashWallpaper {
    type T = UnsplashConfig;

    fn new(config: Self::T) -> Box<Self> {
        let x = UnsplashWallpaper { config };
        Box::new(x)
    }

    fn download_wallpaper(&self, save_path: &PathBuf) -> PathBuf {
        let mut f = save_path.clone();
        f.push("desktop.jpg");

        let save_path: String = String::from(f.to_str().unwrap());
        let ret_save_path = save_path.clone();
        unsplash::get_random_image(save_path);
        PathBuf::from(&ret_save_path[..])
    }

    fn set_wallpaper(&self, picture_path: &PathBuf) {
        let cmd = command_for_desktop_env(picture_path);
        println!("execute command: {}", cmd);
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to exec process");
    }
}

fn command_for_desktop_env(picture_path: &PathBuf) -> String {
    let wallpaper_path = picture_path.to_str().unwrap();
    match env() {
        Gnome => {

            let mut cmd: String = String::from(
                "sqlite3gsettings set org.gnome.desktop.background picture-uri ",
            );
            let _cmd = format!("{}", wallpaper_path);
            cmd.push_str(&_cmd);
            cmd
        },
        Mac => {
            let mut cmd: String = String::from("sqlite3 ");
            cmd.push_str("~/Library/Application\\ Support/Dock/desktoppicture.db ");
            let _cmd = format!(
                "\"update data set value = '{}'\" && killall Dock",
                wallpaper_path
            );
            cmd.push_str(&_cmd);
            cmd
        },
        _ => panic!("unsupported desktop env"),
    }
}
