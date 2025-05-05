#[cfg(windows)]
pub mod wallpaper {
    use windows::{
        Win32::{
            System::Com::{
                CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
            },
            UI::Shell::{DesktopWallpaper, IDesktopWallpaper},
        },
        core::PCWSTR,
    };

    pub fn set_wallpaper(image_path: &str) -> Result<(), windows::core::Error> {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

            let desktop_wallpaper: IDesktopWallpaper =
                CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;

            let wide_image_path: Vec<u16> = image_path.encode_utf16().collect();
            let pcwstr_image_path = PCWSTR(wide_image_path.as_ptr());

            desktop_wallpaper.SetWallpaper(PCWSTR::null(), pcwstr_image_path)?;

            CoUninitialize();

            Ok(())
        }
    }
}

#[cfg(unix)]
pub mod wallpaper {
    pub fn set_wallpaper(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }
}
