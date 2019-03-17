mod wallpaper;

fn main() {
    // create temp directory of unsplashme if not exists.
    let tmp = wallpaper::create_temp_path();
    println!("wallpaper local directory: {:?}", tmp);

    let wallpaper_path = wallpaper::download_wallpaper(tmp);
    println!("saved wallpaper: {:?}", wallpaper_path);

    let final_path = wallpaper::set_wallpaper(wallpaper_path);
    println!("wallpaper set to: {:?}", final_path);
}
