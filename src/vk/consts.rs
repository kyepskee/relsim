use crate::vk::util::DeviceExtension;

pub const APP_NAME: &'static str = "RelSim";
pub const WINDOW_TITLE: &'static str = "RelSim";
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub const DEVICE_EXTENSIONS: DeviceExtension = DeviceExtension {
    names: ["VK_KHR_swapchain"]
};
