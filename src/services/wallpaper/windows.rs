use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use windows::{
    Win32::{
        Foundation::BOOL,
        UI::WindowsAndMessaging::{
            SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SystemParametersInfoW,
        },
    },
    core::PCWSTR,
};

pub fn set_wallpaper(image_path: impl AsRef<Path>) -> Result<(), String> {
    let wide: Vec<u16> = image_path
        .as_ref()
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide.as_ptr() as _),
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        );
        if result == BOOL::from(false) {
            return Err(format!(
                "Unable to set wallpaper at {}",
                image_path.as_ref().display()
            ));
        }
    }
    Ok(())
}
