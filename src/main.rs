use core::borrow::Borrow;
use unsplashme::unsplash::config::UnsplashConfig;
use unsplashme::unsplash::wallpaper::UnsplashWallpaper;
use unsplashme::Wallpaper;

fn main() {
    let config = UnsplashConfig {};
    let wallpaper: Box<UnsplashWallpaper> = UnsplashWallpaper::new(config);
    run(wallpaper.borrow());
}

fn run(wallpaper: &UnsplashWallpaper) {
    let wallpaper: &UnsplashWallpaper = wallpaper.borrow();
    let path = wallpaper.create_temp_path();
    let path = wallpaper.download_wallpaper(&path);
    wallpaper.set_wallpaper(&path);
}
