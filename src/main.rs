mod vk;

use crate::vk::VulkanApp;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let app = VulkanApp::new(&event_loop);
    app.main_loop(event_loop);
}
