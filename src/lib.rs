use angled_random_walker as arw;
use getrandom::getrandom;
use stackblur_iter::{
    blur,
    imgref::{Img, ImgExtMut, ImgRefMut},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Paint {
    Age,
    CumulativeAge,
    Generation,
    Constant,
}

fn scale_age_to_height(age: f64, max_age: f64) -> f64 {
    age / max_age
    //1.0 - (1.0 / (1.0 + linear_scaled * 4.0))
}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn generate(
    size: usize,
    max_long_age: usize,
    max_short_age: usize,
    max_generations: usize,
    children: usize,
    max_long_angle_divergence: f64,
    max_short_angle_divergence: f64,
    paint: Paint,
) -> Vec<u8> {
    let mut seed_byes = [0u8; 8];
    if getrandom(&mut seed_byes).is_err() {
        seed_byes = [1, 2, 3, 4, 5, 6, 7, 8];
    }
    let params = arw::SimulationParams {
        size,
        max_long_age,
        max_short_age,
        max_generations,
        children,
        max_long_angle_divergence,
        max_short_angle_divergence,
        short_branch_frequency: 20,
        paint: match paint {
            Paint::Age => arw::Paint::Age,
            Paint::CumulativeAge => arw::Paint::CumulativeAge,
            Paint::Generation => arw::Paint::Generation,
            Paint::Constant => arw::Paint::Constant,
        },
        initial_walkers: arw::InitialWalkers::CardinalsAndOrdinals,
        seed: u64::from_le_bytes(seed_byes),
    };
    let max_age = max_long_age * (max_generations + 1) + max_generations + 1;
    arw::simulate(params)
        .into_iter()
        .flatten()
        .map(|val| {
            if val == 0 {
                0
            } else {
                (scale_age_to_height((max_age - val as usize) as f64, max_age as f64) * 255.0)
                    .clamp(0.0, 254.0) as u8
            }
        })
        .collect()
}

#[wasm_bindgen]
pub fn to_image(data: Vec<u8>) -> Vec<u8> {
    data.into_iter()
        .flat_map(|val| {
            if val == 0 {
                [0, 0, 0, 0]
            } else {
                [255, 255, 255, 255]
            }
        })
        .collect()
}

#[wasm_bindgen]
pub fn heightmap_blur(data: Vec<u8>, radius: usize) -> Vec<u8> {
    let mut img = Img::new(data, 512, 512);
    blur(&mut img.as_mut(), radius, |p| *p as usize, |p| p as u8);
    img.buf().clone()
}

#[wasm_bindgen]
pub fn hello() -> u32 {
    42
}
