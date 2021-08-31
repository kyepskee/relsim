#![allow(dead_code)]
#![allow(unused_macros)]

#[macro_use]
extern crate static_assertions;

mod consts;
#[macro_use]
mod tensors;
mod physics;
//mod vk;

//use crate::vk::VulkanApp;

fn main() {
    physics::raytrace();
}
