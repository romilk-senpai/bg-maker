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

/*
let image_path = r"C:\wallpaper.jpg";
match set_wallpaper(image_path) {
    Ok(_) => println!("Wallpaper set successfully!"),
    Err(e) => eprintln!("Failed to set wallpaper: {:?}", e),
}
*/
