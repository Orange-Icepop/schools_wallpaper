mod windows;

use std::fs;

fn set_wallpaper(path: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        windows::set_wallpaper(path)?;
    }
    Ok(())
}