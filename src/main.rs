#![allow(dead_code)]
#![allow(unused_macros)]

#[macro_use]
extern crate static_assertions;

mod consts;
#[macro_use]
mod tensors;
mod physics;
mod vk;

use physics::astro_obj::AstroObj;
use tensors::Tensor1;

//use crate::vk::VulkanApp;

//mod color;

#[allow(dead_code)]
fn vulkan() {
    let event_loop = winit::event_loop::EventLoop::new();
    let app = vk::VulkanApp::new(&event_loop);
    app.main_loop(event_loop);
}

fn main() {
    let objs = &[
        AstroObj {
            schwarzschild_radius: 1.0,
            position: Tensor1 { vals: [0.0, 0.0, 0.0, 0.0] },
            radius: 1.0
        }
    ];
    physics::raytrace(objs);
}
