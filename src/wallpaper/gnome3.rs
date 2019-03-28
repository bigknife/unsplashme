use std::process::Command;
use std::str::FromStr;
use std::string::String;
use whoami;

pub fn set_wallpaper<'a>(wallpaper_path: &'a str) {
    match whoami::platform() {
        whoami::Platform::Linux => {
            match whoami::env() {
                whoami::DesktopEnv::Gnome => {
                    //cmd = "gsettings set org.gnome.desktop.background picture-uri file://$HOME/.unsplashme/desktop.jpg"
                    let mut cmd: String = String::from(
                        "sqlite3gsettings set org.gnome.desktop.background picture-uri ",
                    );
                    let _cmd = format!("{}", wallpaper_path);
                    cmd.push_str(&_cmd);

                    //println!("{}", cmd);

                    Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("failed to exec process");
                }
                _ => panic!("not support current env"),
            }
        }
        _ => panic!("only support MacOS"),
    }
}
