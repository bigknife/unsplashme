use std::process::Command;
use std::str::FromStr;
use std::string::String;
use whoami;

pub fn set_wallpaper<'a>(wallpaper_path: &'a str) {
    match whoami::platform() {
        whoami::Platform::MacOS => {
            //     cmd = "sqlite3 ~/Library/Application\\ Support/Dock/desktoppicture.db \"update data set value = '#{path}'\" && killall Dock"
            let mut cmd: String = String::from_str("sqlite3 ").unwrap();
            cmd.push_str("~/Library/Application\\ Support/Dock/desktoppicture.db ");
            let _cmd = format!(
                "\"update data set value = '{}'\" && killall Dock",
                wallpaper_path
            );
            cmd.push_str(&_cmd);

            //println!("{}", cmd);

            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to exec process");
        }
        _ => panic!("only support MacOS"),
    }
}
