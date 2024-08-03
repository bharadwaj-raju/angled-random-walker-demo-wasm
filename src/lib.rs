use angled_random_walker as arw;
use stackblur_iter::{blur, imgref::Img};
use wasm_bindgen::prelude::*;

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
    short_branch_frequency: usize,
    seed_bytes: &[u8],
) -> Vec<u8> {
    let seed_bytes: [u8; 8] = seed_bytes.try_into().unwrap_or([1, 2, 3, 4, 5, 6, 7, 8]);
    let params = arw::SimulationParams {
        size,
        max_long_age,
        max_short_age,
        max_generations,
        children,
        max_long_angle_divergence,
        max_short_angle_divergence,
        short_branch_frequency,
        paint: arw::Paint::CumulativeAge,
        initial_walkers: arw::InitialWalkers::CardinalsAndOrdinals,
        seed: u64::from_le_bytes(seed_bytes),
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
pub fn heightmap_blur(data: Vec<u8>, radius: usize, detail_max: u8) -> Vec<u8> {
    let mut img = Img::new(data.clone(), 512, 512);
    blur(&mut img.as_mut(), radius, |p| *p as usize, |p| p as u8);
    let mut detail_img = Img::new(
        data.iter().map(|d| (*d).clamp(0, detail_max)).collect(),
        512,
        512,
    );
    blur(&mut detail_img.as_mut(), 4, |p| *p as usize, |p| p as u8);
    let first_stage = img.buf().clone();
    first_stage
        .iter()
        .zip(detail_img.buf())
        .map(|(blurred_val, raw_data)| blurred_val + raw_data)
        .collect()
}

#[wasm_bindgen]
pub fn hello() -> u32 {
    42
}
