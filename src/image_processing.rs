use std::collections::HashMap;
use std::io::Read;
use config::Value;
use macroquad::prelude::{Material, Texture2D};
use crate::Application;

use crate::configuration::{ShimmerConfig, ShimmerType};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f32,
}

pub struct DimmerApplicationState {
    pub bit_state: Vec<Vec4f>,
    pub transform: fn(&mut Vec<Vec4f>) -> (),
}

pub fn image_into_bytes(image_path: &String) -> Vec<u8> {

    std::fs::File::open(image_path)
        .map_err(|err| println!("Error:{}", err))
        .map(|mut file| {
            let mut vec: Vec<u8> = vec![];
            file.read_to_end(&mut vec).unwrap();
            vec
        })
/*        .map(|vec| {
            let b_vec = bytemuck::cast_mut::<Vec<u8>, Vec<Vec4f>>(&mut vec); b_vec
        })*/
        .unwrap()
}

impl DimmerApplicationState {

    pub fn new(img_width: usize, config: ShimmerConfig) -> DimmerApplicationState {
        // let mut vec = Vec::<u8>::with_capacity(img_width);
        let mut vec = Vec::<Vec4f>::with_capacity(img_width);

        let func = match config.shimmer_type {
            ShimmerType::NMStripe => { DimmerApplicationState::m_n_stripe_init_state(&mut vec, config.config); DimmerApplicationState::not_switch }
            ShimmerType::Array => { panic!() }
        };

        DimmerApplicationState {
            bit_state: vec,
            transform: func,
        }
    }

    fn m_n_stripe_init_state(state: &mut Vec<Vec4f>, config: HashMap<String, Value>) -> () {
        let n_stripe = config.get("n").unwrap().clone().into_int().unwrap() as usize;
        let m_stripe = config.get("m").unwrap().clone().into_int().unwrap() as usize;

        for i in 0..state.capacity() {
            let pivot = i % (m_stripe + n_stripe);
            let transparency = if pivot < m_stripe { 0.11 } else { 0.88 };
            state.push(Vec4f{ x: transparency, y: 0., z: 0., a: 0. });
        }
    }

    fn not_switch(state: &mut Vec<Vec4f>) {
        for p in state {
            p.x = 1. - p.x;
        }

    }

}
