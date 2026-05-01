mod windows;

use std::fs;

pub fn random_wallpaper(dir: &str) -> Result<String> {
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
        return Err("No bmp images found in the directory".into());
    }

    let random_index = rand::random::<usize>() % entries.len();
    Ok(entries[random_index].path().to_string_lossy().to_string())
}

pub fn set_wallpaper(path: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        windows::set_wallpaper(path)?;
    }
    Ok(())
}