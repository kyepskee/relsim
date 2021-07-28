#![allow(dead_code)]
#![allow(unused_macros)]

#[macro_use]
extern crate static_assertions;

mod consts;
#[macro_use]
mod tensors;
mod vk;

use crate::vk::VulkanApp;

fn main() {
    tensors_sample();
    
    let event_loop = winit::event_loop::EventLoop::new();
    let app = VulkanApp::new(&event_loop);
    app.main_loop(event_loop);
}

fn tensors_sample() {
    use crate::tensors::Tensor2;
    use rand::prelude::*;
    
    let mut rnd: Tensor2 = Default::default();
    let mut rng = rand::thread_rng();
    loop_over!(a, b => {
        rnd.vals[a][b] = rng.gen();
    });
    
    let inv = rnd.inv().unwrap();
    let mut res: Tensor2 = Default::default();
    rnd.inv();
    
    loop_over!(a, b, c => {
        res.vals[a][b] += rnd.vals[a][c] * inv.vals[c][b];
    });
    let trace = add_over!(lambda  => {
        res.vals[lambda][lambda]
    });
    
    println!("trace: {}", trace);
    println!("{:#?}", res);
}
