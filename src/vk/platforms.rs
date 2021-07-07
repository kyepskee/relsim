use ash::vk;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::extensions::khr::Surface;
use ash::extensions::ext::DebugUtils;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(all(windows))]
use ash::extensions::khr::Win32Surface;

use winit::window::Window;

#[cfg(all(windows))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

#[cfg(all(unix, not(target_os = "android")))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &Window
) -> Result<vk::SurfaceKHR, vk::Result> {
    use winit::platform::unix::WindowExtUnix;

    let x11_display = window.xlib_display().unwrap();
    let x11_window = window.xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
        ..Default::default()
    };
    
    let xlib_surface_loader = XlibSurface::new(entry, instance);
    xlib_surface_loader.create_xlib_surface(&x11_create_info, None)
}

#[cfg(all(windows))]
pub unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &Window
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::os::raw::c_void;
    use std::ptr;
    use winit::platform::unix::WindowExtUnix;
    use winapi::shared::windef::HWND;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;

    let hwnd = window.hwnd() as HWND;
    let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
    let win32_create_info = vk::XLibSurfaceCreateInfoKHR {
        hinstance,
        hwnd: hwnd as *const c_void,
        ..Default::default()
    };
    
    unimplemented!()
}
