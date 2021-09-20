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

//mod color;

use crate::tensors::{Tensor1, Tensor2, Tensor3};

use std::io;
use std::io::prelude::*;

use rustbitmap::{BitMap, Rgba};

const RESOLUTION: u32 = 1000;
const DX: f64 = 0.00000001;
//const rs: f64 = 0.1f64;
//const rg: f64 = 0.6f64;

struct Star {
	swarzshild_radius: f64, //someone spell check this
	position: Tensor1,
	radius: f64
}

const stars: [Star; 2] = [
	Star{
		swarzshild_radius: 0.001f64,
		position: Tensor1{vals: [0f64, -3f64, 0f64, 0f64]},
		radius: 0.4f64},
	Star{
		swarzshild_radius: 0.0005f64,
		position: Tensor1{vals: [0f64, 3f64, 0f64, 0f64]},
		radius: 0.3f64}];

fn star_collision (pos: Tensor1) -> bool {
	let mut output: bool = false;
	for star in stars{
		let x_rel = pos.vals[1] - star.position.vals[1];
		let y_rel = pos.vals[2] - star.position.vals[2];
		let z_rel = pos.vals[3] - star.position.vals[3];		
		
		let r = (x_rel*x_rel + y_rel*y_rel + z_rel*z_rel).sqrt();

		if r<=star.radius{
			output = true;
		}
	}
	return output;
}

pub fn g(x: f64, y: f64, z: f64, t: f64) -> Tensor2 {
    //THIS ENTIRE FUNCTION IS STRAIGHT FROM SUSPICOUS LOOKING PORTUGESE PHYSICS PAPER, IF SOMETHING DOESNT WORK CHECK HERE FIRST
    	let mut output = Tensor2::default_minkowski();
	
	for star in stars {	
		let x_rel = x - star.position.vals[1];
		let y_rel = y - star.position.vals[2];
		let z_rel = z - star.position.vals[3];		

	        let r = (x_rel * x_rel + y_rel * y_rel + z_rel * z_rel).sqrt();
       		let A = 1.0f64 - star.swarzshild_radius / r;

	        output.vals[0][0] -= star.swarzshild_radius / r;

	        let mut position = Tensor1::default();

       		position.vals[0] = t;
       		position.vals[1] = x_rel;
       		position.vals[2] = y_rel;
	        position.vals[3] = z_rel;

        	for i in 1..4 {
          	for j in 1..4 {
           	     output.vals[i][j] += position.vals[i]*position.vals[j]/(r*r)* //the term from portugese paper
            	        (-1.0f64/(1f64-star.swarzshild_radius/r)+1.0f64) //the actual term
            	}
        	}
	}
        
	//assert!(output.det()<0);

	return output;

    return Tensor2::default_minkowski();
}

pub fn d_g(x: f64, y: f64, z: f64, t: f64) -> Tensor3 {
    let mut output = Tensor3::default();
    let metric = g(x, y, z, t);
    let g1 = g(x, y, z, t + DX);
    let g2 = g(x + DX, y, z, t);
    let g3 = g(x, y + DX, z, t);
    let g4 = g(x, y, z + DX, t);
    loop_over!(miu, v => {
        let gv = metric.vals[miu][v];
        output.vals[miu][v][0] = (g1.vals[miu][v] - gv)/DX;
        output.vals[miu][v][1] = (g2.vals[miu][v] - gv)/DX;
        output.vals[miu][v][2] = (g3.vals[miu][v] - gv)/DX;
        output.vals[miu][v][3] = (g4.vals[miu][v] - gv)/DX;
    });
    return output;
}

pub fn chris(x: f64, y: f64, z: f64, t: f64) -> Tensor3 {
    let mut output = Tensor3::default();
    let dg = d_g(x, y, z, t);
    let ginv = g(x, y, z, t)
        .inv()
        .expect("Failed to get inverse of g tensor in chris()");

    loop_over!(miu, v, sigma => {
        output.vals[sigma][miu][v] = -0.5f64*add_over!(tau => {
            ginv.vals[sigma][tau] * (
                dg.vals[tau][miu][v] +
                dg.vals[tau][v][miu] -
                dg.vals[miu][v][tau]
            )})
    });
    return output;
}

const RANGE_DOWN: f64 = -5f64;
const RANGE_UP: f64 = 5f64;
const X_AXIS: usize = 1;
const Y_AXIS: usize = 2;

fn pixel_to_vector(x: u32, y: u32) -> Tensor1 {
    let mut output = Tensor1::default();
    output.vals[X_AXIS] = RANGE_UP * (x as f64) / (RESOLUTION as f64)
        + RANGE_DOWN * (RESOLUTION as f64 - x as f64) / (RESOLUTION as f64);
    output.vals[Y_AXIS] = RANGE_UP * (y as f64) / (RESOLUTION as f64)
        + RANGE_DOWN * (RESOLUTION as f64 - y as f64) / (RESOLUTION as f64);
    return output;
}

fn vector_to_pixel_x(pos: Tensor1) -> u32 {
    return (RESOLUTION as f64 * (pos.vals[X_AXIS] - RANGE_DOWN) / (RANGE_UP - RANGE_DOWN)) as u32;
}
fn vector_to_pixel_y(pos: Tensor1) -> u32 {
    return (RESOLUTION as f64 * (pos.vals[Y_AXIS] - RANGE_DOWN) / (RANGE_UP - RANGE_DOWN)) as u32;
}

const SPACING: u32 = 25;
const DT: f64 = 0.01;
const LIMIT: usize = 10000;
const WEAKINING_FACTOR: f64 = 1.0; //someone spell check ths

fn draw_geodesic (x: u32, y: u32, velocity: Tensor1, bitmap: &mut BitMap) {
	let mut pos = pixel_to_vector(x, y);
            let mut vel = velocity;

	//let start_parity = pos.vals[X_AXIS].signum() * pos.vals[Y_AXIS].signum();
	
	let mut counter = 0;

            while !star_collision(pos) && counter < LIMIT {
		counter+=1;
                loop_over! {miu => {
                    pos.vals[miu]+=DT*vel.vals[miu];
                }};
                let chris_sym = chris(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]);
                loop_over! {miu => {
                    vel.vals[miu]+=WEAKINING_FACTOR*add_over!(alpha, beta => {
                        chris_sym.vals[miu][alpha][beta]*vel.vals[alpha]*vel.vals[beta]
                    })
                }};
                (*bitmap).set_pixel(
                    vector_to_pixel_x(pos),
                    vector_to_pixel_y(pos),
                    Rgba::rgb(255u8, 255u8, 255u8),
                );
                //println!("{:#?}", pos);
            }
}

fn main() {
    let mut output = BitMap::new(RESOLUTION, RESOLUTION);
    /*output.set_pixel(
    vector_to_pixel_x(pos),
    vector_to_pixel_y(pos),
        Rgba::rgb(255u8, 255u8, 255u8),
    );*/

	for x in 0..RESOLUTION{
		for y in 0..RESOLUTION{
			let mut pos = pixel_to_vector(x, y);
			pos.vals[0]=0f64;
			let r = add_over!(miu => {pos.vals[miu]*pos.vals[miu]}).sqrt();
			if !star_collision(pixel_to_vector(x, y)){
				output.set_pixel(x, y, Rgba::rgb(0u8, 0u8, 0u8));
			}else{
				output.set_pixel(x, y, Rgba::rgb(255u8, 100u8, 0u8));
			}
		}
	}

    for x in 0..RESOLUTION{
	if x%SPACING == 0 {
		let mut velocity = Tensor1::default();
		velocity.vals[Y_AXIS]=1f64;
		draw_geodesic(x, RESOLUTION/2, velocity, &mut output);
		velocity.vals[Y_AXIS]=-1f64;
		draw_geodesic(x, RESOLUTION/2, velocity, &mut output);
	}
    }

   for y in 0..RESOLUTION{
	if y%SPACING == 0 {
		let mut velocity = Tensor1::default();
		velocity.vals[X_AXIS]=1f64;
		draw_geodesic(RESOLUTION/2, y, velocity, &mut output);
		velocity.vals[X_AXIS]=-1f64;
		draw_geodesic(RESOLUTION/2, y, velocity, &mut output);
	}
    }

    let mut velocity = Tensor1::default();
    
    velocity.vals[Y_AXIS] = 1f64;
    draw_geodesic(RESOLUTION/2, 0, velocity, &mut output);

    velocity.vals[Y_AXIS] = -1f64;
    draw_geodesic(RESOLUTION/2, RESOLUTION-1, velocity, &mut output);
    velocity.vals[Y_AXIS] = 0f64;

    velocity.vals[X_AXIS] = 1f64;
    draw_geodesic(0, RESOLUTION/2, velocity, &mut output);

    velocity.vals[X_AXIS] = -1f64;
    draw_geodesic(RESOLUTION-1, RESOLUTION/2, velocity, &mut output);

    output.save_as("output_geodesic.bmp").unwrap();
}
