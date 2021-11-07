use std::collections::HashMap;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use config::Value;

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
    pub misc_state: Box<u128>,
    pub transform: fn(&mut Vec<Vec4f>, &HashMap<String, Value>, u128) -> (),
}

pub fn image_into_bytes(image_path: &String) -> Vec<u8> {

    std::fs::File::open(image_path)
        .map_err(|err| println!("Error:{}", err))
        .map(|mut file| {
            let mut vec: Vec<u8> = vec![];
            file.read_to_end(&mut vec).unwrap();
            vec
        })
        .unwrap()
}

impl DimmerApplicationState {

    pub fn new(img_width: usize, config: &ShimmerConfig) -> DimmerApplicationState {
        let mut vec = Vec::<Vec4f>::with_capacity(img_width);
        let mut misc_state = Box::new(0);

        let func: fn(&mut Vec<Vec4f>, config: &HashMap<String, Value>, u128) = match config.shimmer_type {
            ShimmerType::NMStripe => {
                DimmerApplicationState::m_n_stripe_init_state(&mut vec, &config.config, &mut misc_state); DimmerApplicationState::not_switch }
            ShimmerType::Array => {
                DimmerApplicationState::array_init_state(&mut vec, &config.config, &mut misc_state); DimmerApplicationState::controlled_switch }
        };

        DimmerApplicationState {
            bit_state: vec,
            misc_state,
            transform: func,
        }
    }

    fn m_n_stripe_init_state(state: &mut Vec<Vec4f>, config: &HashMap<String, Value>,  _: &mut Box<u128>) -> () {
        let n_stripe = config.get("n").unwrap().clone().into_int().unwrap() as usize;
        let m_stripe = config.get("m").unwrap().clone().into_int().unwrap() as usize;

        for i in 0..state.capacity() {
            let pivot = i % (m_stripe + n_stripe);
            let transparency = if pivot < m_stripe { 0.11 } else { 0.88 };
            state.push(Vec4f{ x: transparency, y: 0., z: 0., a: 0. });
        }
    }

    fn not_switch(state: &mut Vec<Vec4f>, _: &HashMap<String, Value>, _: u128) {
        for p in state {
            p.x = 1. - p.x;
        }

    }

    fn array_init_state(state: &mut Vec<Vec4f>, _: &HashMap<String, Value>, misc_state: &mut Box<u128>) -> () {

        **misc_state = DimmerApplicationState::curr_msecs();

        for _ in 0..state.capacity() {
            state.push(Vec4f{x:1.0, y: 0., z: 0., a: 0.} )
        }

    }

    fn controlled_switch(bit_state: &mut Vec<Vec4f>,  config: &HashMap<String, Value>, misc_state: u128) {
        let curr_time = DimmerApplicationState::curr_msecs();
        let config_array = config.get("array").unwrap().to_owned();
        let config_array = config_array.into_array().unwrap();
        let stripe_count = config.get("stripes_count").unwrap().to_owned().into_int().unwrap();
        let strip_wight = bit_state.len() / stripe_count as usize;
        let time_passed: u128 = curr_time - misc_state;

        for (i, stripe) in (0..config_array.len()).zip(config_array) {
            let stripe_config = stripe.into_table().unwrap();
            let time = stripe_config.get("time").unwrap().to_owned().into_int().unwrap();
            let duration = stripe_config.get("delay").unwrap().to_owned().into_int().unwrap();
            let phase_duration = (time + duration) as u128;
            let phase = time_passed % phase_duration;
            let transparency = stripe_config.get("trans").unwrap().to_owned().into_float().unwrap();

            for j in 00..strip_wight {
                let mut vec = &mut bit_state[i * strip_wight + j];
                vec.x = if phase > time as u128 { 0. } else { transparency as f32 };
            }
        }
    }

    fn curr_msecs() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}
