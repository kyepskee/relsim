mod color;

use crate::tensors::{Tensor1, Tensor2, Tensor3};

use std::io;
use std::io::prelude::*;

use rustbitmap::{BitMap, Rgba};

const RESOLUTION: u32 = 1000;

const LOWER_BOUND: f64 = -10f64;
const UPPER_BOUND: f64 = 10f64;
const DISTANCE_TO_SCREEN: f64 = 10f64;

const INNER_RADIUS: f64 = 2f64;
const OUTER_RADIUS: f64 = 5f64;
const THICKNESS: f64 = 0.1f64;
const SURFACE: Tensor1 = Tensor1 {
    vals: [0f64, 0f64, 1f64, -0.1f64],
};

const SKY_BOX: f64 = 20f64;

const DX: f64 = 0.00000001f64;
const DT: f64 = 0.01f64;

const rs: f64 = 1.0f64;

pub fn g(x: f64, y: f64, z: f64, t: f64) -> Tensor2 {
    //THIS ENTIRE FUNCTION IS STRAIGHT FROM SUSPICOUS LOOKING PORTUGESE PHYSICS PAPER, IF SOMETHING DOESNT WORK CHECK HERE FIRST

    let r = (x * x + y * y + z * z).sqrt();
    let A = 1.0f64 - rs / r;
    let mut output = Tensor2::default();

    output.vals[0][0] = A;

    let mut position = Tensor1::default();

    position.vals[0] = t;
    position.vals[1] = x;
    position.vals[2] = y;
    position.vals[3] = z;

    for i in 1..4 {
        for j in 1..4 {
            output.vals[i][j] = position.vals[i]*position.vals[j]/(r*r)* //the term from portugese paper
				(-1.0f64/A+1.0f64) //the actual term
				-{if i==j {1f64} else {0f64}}; // + delta
        }
    }

    return output;
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

fn collision(pos: Tensor1, vel: Tensor1, start_pos: Tensor1, start_vel: Tensor1) -> Option<Rgba> {
    let r =
        (pos.vals[1] * pos.vals[1] + pos.vals[2] * pos.vals[2] + pos.vals[3] * pos.vals[3]).sqrt();
    if r >= SKY_BOX {
        return Some(Rgba::rgb(0u8, 0u8, 100u8));
    } else if r <= rs * (1.0 + 2.0 * DT) {
        return Some(Rgba::rgb(0u8, 0u8, 0u8));
    } else if r <= OUTER_RADIUS
        && r >= INNER_RADIUS
        && add_over!(miu => { pos.vals[miu] * SURFACE.vals[miu] }).abs() <= THICKNESS
    {
        let mut wavelength = 590f64;
        let mut amplitude = 1f64;

        /*let mut velocity_prime = vel;
        velocity_prime.vals[0] = 0f64;

        let mut start_velocity_prime = start_vel;
        start_velocity_prime.vals[0] = 0f64;

        wavelength = wavelength
            * add_over!(miu, v => {
                g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]).vals[miu][v] *
                start_velocity_prime.vals[miu] *
                start_velocity_prime.vals[v]
            })
            / add_over!(miu, v => {
                g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]).vals[miu][v] *
                velocity_prime.vals[miu] *
                velocity_prime.vals[v]
            });*/

        return Some(Rgba::rgb(
            (wavelength / 590f64 * 200f64) as u8,
            (wavelength / 590f64 * 100f64) as u8,
            (wavelength / 590f64 * 100f64) as u8,
        ));
    }
    return None;
}

fn get_initial_position(x: u32, y: u32) -> Tensor1 {
    let mut output = Tensor1::default();
    output.vals[1] =
        LOWER_BOUND + (x as f64) / ((RESOLUTION - 1) as f64) * (UPPER_BOUND - LOWER_BOUND);
    output.vals[2] =
        LOWER_BOUND + (y as f64) / ((RESOLUTION - 1) as f64) * (UPPER_BOUND - LOWER_BOUND);
    output.vals[3] = DISTANCE_TO_SCREEN;
    return output;
}

pub fn raytrace() {
    let mut output = BitMap::new(RESOLUTION, RESOLUTION);

    println!("STARTING RENDER :)");

    //let x = 5;
    //let y = 5;

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let start_pos = get_initial_position(x, y);
            let mut pos = start_pos;

            let mut velocity = Tensor1::default();
            velocity.vals[0] = 10f64;
            velocity.vals[3] = -velocity.vals[0]
                * (-g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]).vals[0][0]
                    / g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]).vals[3][3])
                    .sqrt(); //TODO: this part now only works assuming g_0_3=0, fix it for the future.

            let start_velocity = velocity;

            let mut color: Option<Rgba> = None;
            while color.is_none() {
                pos.vals[0] = 0f64;

                loop_over!(miu => {
                    pos.vals[miu] += DT * velocity.vals[miu]
                });

                let christoffel = chris(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0]);
                loop_over!(miu => {
                    velocity.vals[miu] += DT *
                        add_over!(alpha, beta => {
                            christoffel.vals[miu][alpha][beta] * velocity.vals[alpha] * velocity.vals[beta]
                        })
                });

                color = collision(pos, velocity, start_pos, start_velocity);
            }
            //println!("outputing: {:#?}", color);
            output.set_pixel(x, y, color.unwrap());
            if y % 100 == 0 {
                print!(
                    "DONE IN {:.2}%            \r",
                    ((x as f64) / (RESOLUTION as f64) + (y as f64) / (RESOLUTION.pow(2) as f64))
                        * 100.0
                );
                io::stdout().flush();
            }
        }
    }

    // for i in 0..RESOLUTION {
    //     let t = 1.0 - (i as f64) / (RESOLUTION as f64);
    //     let freq = 400.0 * t + 700.0 * (1.0 - t);
    //     println!("{}", freq);
    //     for j in 0..RESOLUTION {
    //         output
    //             .set_pixel(i, j, color::freq_to_rgb(freq, (j as f64)/(RESOLUTION as f64)))
    //             .unwrap();
    //     }
    // }
    output.save_as("output.bmp").unwrap();
}
