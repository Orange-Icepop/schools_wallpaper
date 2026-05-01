mod services;
mod handle;

use services::*;
use crate::handle::ResultExt;

fn main() {
    println!("Hello, world!");
    println!("Reading config...");
    let config = configs::read_all_config().unwrap_or_log();
    if config.use_override_wallpaper {
        println!("Using override wallpaper: {}", config.override_wallpaper);
        wallpaper::set_wallpaper(&config.override_wallpaper).unwrap_or_log();
        return;
    }
    let weekday = date::get_weekday();
    println!("Today is day {} of the week.", weekday);
    wallpaper::set_today_wallpaper(&config.wallpaper_dir, weekday).unwrap_or_log();
    println!("Wallpaper set successfully.");
}