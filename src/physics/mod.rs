pub mod astro_obj;
mod color;

use crate::tensors::{Tensor1, Tensor2, Tensor3};
use astro_obj::AstroObj;

use std::io;
use std::io::prelude::*;

use rustbitmap::{BitMap, Rgba};

const RESOLUTION: u32 = 1000;

const LOWER_BOUND: f64 = -10f64;
const UPPER_BOUND: f64 = 10f64;
const DISTANCE_TO_SCREEN: f64 = 10f64;

const SPACING: u32 = 25;
const DT: f64 = 0.01;
const LIMIT: usize = 10000;
const WEAKENING_FACTOR: f64 = 1.0;

const INNER_RADIUS: f64 = 2f64; //according to my calculations the smallest stable orbit is 1.5*r_s (double check that)
const OUTER_RADIUS: f64 = 10f64;
const THICKNESS: f64 = 0.1f64;
const SURFACE: Tensor1 = Tensor1 {
    vals: [0f64, 0f64, 1f64, -0.1f64],
};

const RANGE_DOWN: f64 = -5f64;
const RANGE_UP: f64 = 5f64;
const X_AXIS: usize = 1;
const Y_AXIS: usize = 2;

const SKY_BOX: f64 = 20f64;

const DX: f64 = 0.00000001f64;

const rs: f64 = 1.0f64;

fn star_collision(pos: Tensor1, objs: &[AstroObj]) -> bool {
    for obj in objs {
        let x_rel = pos.vals[1] - obj.position.vals[1];
        let y_rel = pos.vals[2] - obj.position.vals[2];
        let z_rel = pos.vals[3] - obj.position.vals[3];

        let r = (x_rel * x_rel + y_rel * y_rel + z_rel * z_rel).sqrt();

        if r <= obj.radius {
            return true;
        }
    }
    false
}

pub fn g(x: f64, y: f64, z: f64, t: f64, objs: &[AstroObj]) -> Tensor2 {
    let mut output = Tensor2::default_minkowski();

    for obj in objs {
        let x_rel = x - obj.position.vals[1];
        let y_rel = y - obj.position.vals[2];
        let z_rel = z - obj.position.vals[3];

        let r = (x_rel * x_rel + y_rel * y_rel + z_rel * z_rel).sqrt();
        let A = 1.0f64 - obj.schwarzschild_radius / r;

        output.vals[0][0] -= obj.schwarzschild_radius / r;

        let mut position = Tensor1::default();

        position.vals[0] = t;
        position.vals[1] = x_rel;
        position.vals[2] = y_rel;
        position.vals[3] = z_rel;

        for i in 1..4 {
            for j in 1..4 {
                output.vals[i][j] += position.vals[i]*position.vals[j]/(r*r)* //the term from portugese paper
                    (-1.0f64/(1f64-obj.schwarzschild_radius/r)+1.0f64) //the actual term
            }
        }

    }

    // assert!(output.det()<0);

    output
    // return Tensor2::default_minkowski();
}

pub fn d_g(x: f64, y: f64, z: f64, t: f64, objs: &[AstroObj]) -> Tensor3 {
    let mut output = Tensor3::default();
    let metric = g(x, y, z, t, objs);

    let g1 = g(x, y, z, t + DX, objs);
    let g2 = g(x + DX, y, z, t, objs);
    let g3 = g(x, y + DX, z, t, objs);
    let g4 = g(x, y, z + DX, t, objs);

    loop_over!(miu, v => {
        let gv = metric.vals[miu][v];
        output.vals[miu][v][0] = (g1.vals[miu][v] - gv)/DX;
        output.vals[miu][v][1] = (g2.vals[miu][v] - gv)/DX;
        output.vals[miu][v][2] = (g3.vals[miu][v] - gv)/DX;
        output.vals[miu][v][3] = (g4.vals[miu][v] - gv)/DX;
    });

    output
}

pub fn chris(x: f64, y: f64, z: f64, t: f64, objs: &[AstroObj]) -> Tensor3 {
    let mut output = Tensor3::default();
    let dg = d_g(x, y, z, t, objs);
    let ginv = g(x, y, z, t, objs)
        .inv()
        .expect("Failed to get inverse of g tensor in chris()");

    loop_over!(miu, v, sigma => {
        output.vals[sigma][miu][v] = -0.5f64 * add_over!(tau => {
            ginv.vals[sigma][tau] * (
                dg.vals[tau][miu][v] +
                dg.vals[tau][v][miu] -
                dg.vals[miu][v][tau]
            )})
    });

    output
}

fn doppler(
    g: Tensor2,
    source_velocity: Tensor1,
    ray_velocity: Tensor1,
    proper_time_period: f64,
) -> f64 {
    //this function takes the period as expierienced by source and outputs pieriod as time-coordinate interval
    //this function is this time straight from paper about special relativity (the formulation without tensors) so if physics seems incorrect check this function
    //strictly speaking light doesnt have a 4-velocity so ray_velocity should be called the smart word for it but A: i dont remember it B: it would be confusing
    //also this function propably assumes some type of symmetry (continous time translation i think) but at this point i dont care

    let mut source_velocity_3d = source_velocity;
    let mut ray_velocity_3d = ray_velocity;

    let normalization_factor_source = g.vals[0][0].sqrt() * source_velocity_3d.vals[0];
    let normalization_factor_ray = g.vals[0][0].sqrt() * ray_velocity_3d.vals[0];

    loop_over!(miu => {
        source_velocity_3d.vals[miu]=source_velocity_3d.vals[miu]/normalization_factor_source;
        ray_velocity_3d.vals[miu]=ray_velocity_3d.vals[miu]/normalization_factor_ray;
    });

    source_velocity_3d.vals[0] = 0f64;
    ray_velocity_3d.vals[0] = 0f64;

    let source_vel_length: f64 = add_over!(miu, v => {g.vals[miu][v]*source_velocity_3d.vals[miu]*source_velocity_3d.vals[v]}).abs().sqrt();
    let ray_vel_length: f64 =
        add_over!(miu, v => {g.vals[miu][v]*ray_velocity_3d.vals[miu]*ray_velocity_3d.vals[v]})
            .abs()
            .sqrt();

    let dot_product: f64 =
        add_over!(miu, v => {g.vals[miu][v]*source_velocity_3d.vals[miu]*ray_velocity_3d.vals[v]});

    let cos_theta = dot_product / (source_vel_length * ray_vel_length); //angle between the two vectors
                                                                        //its actually phi' int the paper becouse of the inversed frames of refrance

    let cos_theta_prime = (cos_theta - source_vel_length) / (1.0 - source_vel_length * cos_theta);
    //let cos_theta_prime = (cos_theta+source_vel_length)/(1.0+source_vel_length*cos_theta); //angle of the ray in the "stationary" frame of refrence
    //POSSIBLE SIGN ERROR HERE

    let output_period = proper_time_period * ((1.0 - source_vel_length * source_vel_length).sqrt())
        / (1.0 - cos_theta_prime * source_vel_length);

    //return proper_time_period*(1.0-0.1f64*cos_theta);
    return output_period;
}

fn pixel_to_vector(x: u32, y: u32) -> Tensor1 {
    let mut output = Tensor1::default();

    output.vals[X_AXIS] = RANGE_UP * (x as f64) / (RESOLUTION as f64)
        + RANGE_DOWN * (RESOLUTION as f64 - x as f64) / (RESOLUTION as f64);
    output.vals[Y_AXIS] = RANGE_UP * (y as f64) / (RESOLUTION as f64)
        + RANGE_DOWN * (RESOLUTION as f64 - y as f64) / (RESOLUTION as f64);

    output
}

fn vector_to_pixel_x(pos: Tensor1) -> u32 {
    (RESOLUTION as f64 * (pos.vals[X_AXIS] - RANGE_DOWN) / (RANGE_UP - RANGE_DOWN)) as u32
}

fn vector_to_pixel_y(pos: Tensor1) -> u32 {
    (RESOLUTION as f64 * (pos.vals[Y_AXIS] - RANGE_DOWN) / (RANGE_UP - RANGE_DOWN)) as u32
}

fn doppler_amplitude(
    g: Tensor2,
    source_velocity: Tensor1,
    ray_velocity: Tensor1,
    proper_time_period: f64,
) -> f64 {
    //returns scaling factor for amplitude

    let mut source_velocity_3d = source_velocity;
    let mut ray_velocity_3d = ray_velocity;

    let normalization_factor_source = g.vals[0][0].sqrt() * source_velocity_3d.vals[0];
    let normalization_factor_ray = g.vals[0][0].sqrt() * ray_velocity_3d.vals[0];

    loop_over!(miu => {
        source_velocity_3d.vals[miu]=source_velocity_3d.vals[miu]/normalization_factor_source;
        ray_velocity_3d.vals[miu]=ray_velocity_3d.vals[miu]/normalization_factor_ray;
    });

    source_velocity_3d.vals[0] = 0f64;
    ray_velocity_3d.vals[0] = 0f64;

    let source_vel_length: f64 = add_over!(miu, v => {g.vals[miu][v]*source_velocity_3d.vals[miu]*source_velocity_3d.vals[v]}).abs().sqrt();
    let ray_vel_length: f64 =
        add_over!(miu, v => {g.vals[miu][v]*ray_velocity_3d.vals[miu]*ray_velocity_3d.vals[v]})
            .abs()
            .sqrt();

    let dot_product: f64 =
        add_over!(miu, v => {g.vals[miu][v]*source_velocity_3d.vals[miu]*ray_velocity_3d.vals[v]});

    let cos_theta = dot_product / (source_vel_length * ray_vel_length); //angle between the two vectors
                                                                        //its actually phi' int the paper becouse of the inversed frames of refrance

    let mut cos_theta_prime =
        (cos_theta - source_vel_length) / (1.0 - source_vel_length * cos_theta);
    //let cos_theta_prime = (cos_theta+source_vel_length)/(1.0+source_vel_length*cos_theta); //angle of the ray in the "stationary" frame of refrence
    //POSSIBLE SIGN ERROR HERE

    //println!("cos_theta: {}, v/c: {}", cos_theta_prime, source_vel_length);

    return (1.0 - cos_theta_prime * source_vel_length)
        / ((1.0f64 - source_vel_length * source_vel_length).sqrt());
}

fn collision(
    pos: Tensor1,
    vel: Tensor1,
    start_pos: Tensor1,
    start_vel: Tensor1,
    objs: &[AstroObj],
) -> Option<Rgba> {
    let r =
        (pos.vals[1] * pos.vals[1] + pos.vals[2] * pos.vals[2] + pos.vals[3] * pos.vals[3]).sqrt();
    if r >= SKY_BOX {
        return Some(Rgba::rgb(0u8, 0u8, 0u8 /*100u8*/));
    } else if r <= rs * (1.0 + 2.0 * DT) {
        return Some(Rgba::rgb(0u8, 0u8, 0u8));
    } else if r <= OUTER_RADIUS
        && r >= INNER_RADIUS
        && add_over!(miu => { pos.vals[miu] * SURFACE.vals[miu] }).abs() <= THICKNESS
    {
        let local_g = g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0], objs);

        //let mut frequency = 500f64;
        let mut amplitude = 0.5f64;

        let mut source_velocity = pos;

        source_velocity.vals[0] = (1f64 / local_g.vals[0][0].abs()).sqrt();
        source_velocity.vals[3] = pos.vals[1];
        source_velocity.vals[1] = -pos.vals[3];
        source_velocity.vals[2] = 0f64;

        let mut normalization_factor = 0f64;

        for i in 1..4 {
            for j in 1..4 {
                normalization_factor +=
                    local_g.vals[i][j] * source_velocity.vals[i] * source_velocity.vals[j];
            }
        }

        normalization_factor = normalization_factor.abs().sqrt();

        for i in 1..4 {
            source_velocity.vals[i] =
                source_velocity.vals[i] * (rs / (2.0 * r)).sqrt() / normalization_factor;
        }

        let mut period = 600f64; //1f64/frequency;

        let mut ray_vel = vel;

        loop_over!(miu => {ray_vel.vals[miu] = -ray_vel.vals[miu]});

        //println!("DEBUG, SOURCE_VELOCITY: {:#?}", source_velocity);

        period = doppler(local_g.clone(), source_velocity, ray_vel, period);
        amplitude =
            amplitude * doppler_amplitude(local_g.clone(), source_velocity, ray_vel, period);

        period = g(
            start_pos.vals[1],
            start_pos.vals[2],
            start_pos.vals[3],
            start_pos.vals[0],
            objs,
        )
        .vals[0][0]
            .abs()
            .sqrt()
            / local_g.vals[0][0].abs().sqrt()
            * period;
        //local_g.vals[0][0].abs().sqrt()/g(start_pos.vals[1], start_pos.vals[2], start_pos.vals[3], start_pos.vals[0]).vals[0][0].abs().sqrt()*period;

        //SCALING PERIOD AROUND MIDDLE OF THE SCALE

        period = 0.3f64 * (period - 500f64) + 500f64; //this line is chosen to look good :)

        return Some(color::freq_to_rgb(
            period,
            0.5f64 + 2f64 * (amplitude - 0.5f64),
        )); //the 4 factor in amplitude is chosen to look nice :)
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

fn draw_geodesic(x: u32, y: u32, velocity: Tensor1, bitmap: &mut BitMap, objs: &[AstroObj]) {
    let mut pos = pixel_to_vector(x, y);
    let mut vel = velocity;

    // let start_parity = pos.vals[X_AXIS].signum() * pos.vals[Y_AXIS].signum();

    let mut counter = 0;

    // while !star_collision(pos, objs) && counter < LIMIT {
    while counter < LIMIT {
        counter += 1;
        loop_over! {miu => {
            pos.vals[miu]+=DT*vel.vals[miu];
        }};
        let chris_sym = chris(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0], objs);
        loop_over!(miu => {
            vel.vals[miu] += WEAKENING_FACTOR * add_over!(alpha, beta => {
                chris_sym.vals[miu][alpha][beta] * vel.vals[alpha] * vel.vals[beta]
            })
        });
        (*bitmap)
            .set_pixel(
                vector_to_pixel_x(pos),
                vector_to_pixel_y(pos),
                Rgba::rgb(255u8, 255u8, 255u8),
            ).ok();
        //println!("{:#?}", pos);
    }
}

pub fn raytrace(objs: &[AstroObj]) {
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
                * (-g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0], objs).vals[0][0]
                    / g(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0], objs).vals[3][3])
                    .sqrt();
            // TODO: this part now only works assuming g_0_3=0, fix it for the future.

            let start_velocity = velocity;

            let mut color: Option<Rgba> = None;
            while color.is_none() {
                pos.vals[0] = 0f64;

                loop_over!(miu => {
                    pos.vals[miu] += DT * velocity.vals[miu]
                });

                let christoffel = chris(pos.vals[1], pos.vals[2], pos.vals[3], pos.vals[0], objs);
                loop_over!(miu => {
                    velocity.vals[miu] += DT *
                        add_over!(alpha, beta => {
                            christoffel.vals[miu][alpha][beta] * velocity.vals[alpha] * velocity.vals[beta]
                        })
                });

                color = collision(pos, velocity, start_pos, start_velocity, objs);
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

pub fn geodesic_raytrace(objs: &[AstroObj]) {
    let mut output = BitMap::new(RESOLUTION, RESOLUTION);

    // draw a simple picture, with a black background and orange when you hit a star
    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let mut pos = pixel_to_vector(x, y);
            pos.vals[0] = 0f64;

            // was a star hit from this pixel?
            if !star_collision(pixel_to_vector(x, y), objs) {
                output
                    .set_pixel(x, y, Rgba::rgb(0u8, 0u8, 0u8))
                    .expect("Failed to draw background pixel");
            } else {
                output
                    .set_pixel(x, y, Rgba::rgb(255u8, 100u8, 0u8))
                    .expect("Failed to draw star pixel");
            }
        }
    }

    // draw geodesics with `SPACING` pixels between them
    for x in 0..RESOLUTION {
        if x % SPACING == 0 {
            let mut velocity = Tensor1::default();
            velocity.vals[Y_AXIS] = 1f64;
            draw_geodesic(x, RESOLUTION / 2, velocity, &mut output, objs);
            velocity.vals[Y_AXIS] = -1f64;
            draw_geodesic(x, RESOLUTION / 2, velocity, &mut output, objs);
        }
    }

    for y in 0..RESOLUTION {
        if y % SPACING == 0 {
            let mut velocity = Tensor1::default();
            velocity.vals[X_AXIS] = 1f64;
            draw_geodesic(RESOLUTION / 2, y, velocity, &mut output, objs);
            velocity.vals[X_AXIS] = -1f64;
            draw_geodesic(RESOLUTION / 2, y, velocity, &mut output, objs);
        }
    }

    let mut velocity = Tensor1::default();

    // draw special geodesics
    velocity.vals[Y_AXIS] = 1f64;
    draw_geodesic(RESOLUTION / 2, 0, velocity, &mut output, objs);

    velocity.vals[Y_AXIS] = -1f64;
    draw_geodesic(RESOLUTION / 2, RESOLUTION - 1, velocity, &mut output, objs);
    velocity.vals[Y_AXIS] = 0f64;

    velocity.vals[X_AXIS] = 1f64;
    draw_geodesic(0, RESOLUTION / 2, velocity, &mut output, objs);

    velocity.vals[X_AXIS] = -1f64;
    draw_geodesic(RESOLUTION - 1, RESOLUTION / 2, velocity, &mut output, objs);

    output.save_as("output_geodesic.bmp").unwrap();
}
