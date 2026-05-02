#[cfg(target_os = "windows")]
mod windows;

use anyhow::{bail, Result};
use rand::RngExt;
use std::fs;
use std::path::{Path, PathBuf};

pub fn set_today_wallpaper(img_root_dir: &str, weekday: u32) -> Result<()> {
    if weekday == 0 || weekday > 7 {
        bail!("Invalid date: {}. Date should be between 1 and 7.", weekday);
    }
    let wallpaper_path = random_wallpaper(
        Path::new(img_root_dir)
            .join(weekday.to_string())
            .as_mut_os_str(),
    )?;
    set_wallpaper(wallpaper_path)?;
    Ok(())
}

fn random_wallpaper(dir: impl AsRef<Path>) -> Result<PathBuf> {
    let entries = fs::read_dir(&dir)?
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
        bail!(
            "No bmp images found in the directory {}",
            dir.as_ref().display()
        );
    }

    let random_index = rand::rng().random_range(0..entries.len());
    Ok(entries[random_index].path())
}

pub fn set_wallpaper(path: impl AsRef<Path>) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        windows::set_wallpaper(path)?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        bail!(
            "Setting wallpaper is not supported on this platform: {}",
            std::env::consts::OS
        );
    }
}
