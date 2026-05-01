use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::BOOL,
        UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDWININICHANGE},
    },
};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

fn set_wallpaper(image_path: &str) -> Result<()> {
    // 将路径转换为 UTF-16 并以 null 结尾
    let path_wide: Vec<u16> = OsStr::new(image_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        // 参数：SPI_SETDESKWALLPAPER, 0, 路径指针, SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE
        let result = SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(path_wide.as_ptr() as _),
            // 立即更新并广播更改通知
            windows::Win32::UI::WindowsAndMessaging::SPIF_UPDATEINIFILE
                | windows::Win32::UI::WindowsAndMessaging::SPIF_SENDWININICHANGE,
        );
        if result == BOOL::from(false) {
            return Err("Unable to set wallpaper using winapi".into());
        }
    }
    Ok(())
}