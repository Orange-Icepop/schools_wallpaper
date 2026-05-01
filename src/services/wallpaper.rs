#[cfg(target_os = "windows")]
mod windows;

use rand;
use std::{fs, fs::DirEntry, path::Path, path::PathBuf};

pub fn set_today_wallpaper(img_root_dir: &str, date: u32) -> Result<()> {
    if date == 0 || date > 7 {
        return Err(format!(
            "Invalid date: {}. Date should be between 1 and 7.",
            date
        ));
    }
    let wallpaper_path = random_wallpaper(Path::new(img_root_dir).join(date).as_mut_os_str())?;
    set_wallpaper(wallpaper_path)?;
    Ok(())
}

fn random_wallpaper(dir: impl AsRef<Path>) -> Result<PathBuf> {
    let entries = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            if let Some(ext) = e.path().extension() {
                ext.eq_ignore_ascii_case("bmp")
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        return Err(format!(
            "No bmp images found in the directory {}",
            dir.as_ref().display()
        ));
    }

    let random_index = rand::random::<usize>() % entries.len();
    Ok(entries[random_index].path())
}

pub fn set_wallpaper(path: impl AsRef<Path>) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        windows::set_wallpaper(path)?;
        Ok(())
    }
    Err(format!(
        "Setting wallpaper is not supported on this platform: {}",
        std::env::consts::OS
    ))
}
