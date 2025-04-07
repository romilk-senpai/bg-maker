use windows::{
    core::PCWSTR,
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
        },
        UI::Shell::{DesktopWallpaper, IDesktopWallpaper},
    },
};

pub fn set_wallpaper(image_path: &str) -> Result<(), windows::core::Error> {
    unsafe {
        // Initialize COM library
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        // Create an instance of the DesktopWallpaper COM object
        let desktop_wallpaper: IDesktopWallpaper =
            CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL)?;

        // Convert the string path to wide string (UTF-16)
        let wide_image_path: Vec<u16> = image_path.encode_utf16().collect();
        let pcwstr_image_path = PCWSTR(wide_image_path.as_ptr());

        // Call the SetWallpaper method to set the wallpaper
        desktop_wallpaper.SetWallpaper(PCWSTR::null(), pcwstr_image_path)?;

        // Clean up COM library
        CoUninitialize();

        Ok(())
    }
}
