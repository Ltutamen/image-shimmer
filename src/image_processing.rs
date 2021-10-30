use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Read;
use config::Value;

use crate::configuration::{WinConfig, ShimmerConfig, ShimmerType};

pub fn image_into_bytes(image_path: &String) -> Vec<u8> {

    std::fs::File::open(image_path)
        .map_err(|err| println!("Error:{}", err))
        .map(|mut file| {
            let mut vec: Vec<u8> = vec![];
            match file.read_to_end(&mut vec) {
                Ok(_) => {}
                Err(e) => {println!("Error:{}", e)}
            }
            vec})
        .unwrap()
}

pub struct DimmerApplicationState {
    pub bit_state: Vec<u8>,
    pub transform: fn(&mut Vec<u8>) -> (),
}

impl DimmerApplicationState {

    pub fn new(img_width: usize) -> DimmerApplicationState {

        //  4 u8 in f32, 4 f32 in color
        let mut bit_state = Vec::<u8>::with_capacity(img_width * 4 * 4);

        //  todo write start state
        for i in 0..img_width {
            DimmerApplicationState::push_stripe(&mut bit_state, i % 2 == 0);
        }

        DimmerApplicationState {
            bit_state,
            transform: |vec| -> () {
                for i in vec {
                    if *i > 0 {
                        *i = 0;
                    } else {
                        *i = 144;
                    }
                }
            }
        }
    }

    pub fn from_config(img_width: usize, config: ShimmerConfig) -> DimmerApplicationState {
        let mut vec = Vec::<u8>::with_capacity(img_width);

        let func = match config.shimmer_type {
            ShimmerType::NMStripe => { DimmerApplicationState::m_n_stripe_init_state(&mut vec, config.config); DimmerApplicationState::not_switch }
        };

        DimmerApplicationState {
            bit_state: vec,
            transform: func,
        }
    }

    pub fn switch(&mut self) {
        let func = self.transform;
        let new_state = func(&mut self.bit_state);
    }

    fn set_stripe(&mut self, pos: usize, active: bool) {
        self.bit_state[pos * 16] = 144;
    }
    
    fn push_stripe(bit_state: &mut Vec<u8>, is_stripe: bool) {
        if is_stripe {
            bit_state.push(144);
        } else {
            bit_state.push(0);
        }
        bit_state.push(0);
        bit_state.push(0);
        bit_state.push(0);
        &bit_state;
    }

    fn m_n_stripe_init_state(state: &mut Vec<u8>, config: HashMap<String, Value>) -> () {
        let n_stripe = config.get("n").unwrap().clone().into_int().unwrap() as usize;
        let m_stripe = config.get("m").unwrap().clone().into_int().unwrap() as usize;

        for i in 0..state.capacity() {
            let pivot = i % (m_stripe + n_stripe);
            DimmerApplicationState::push_stripe(state, pivot < m_stripe);
        }
    }

    fn not_switch(state: &mut Vec<u8>) {
        for p in state {
            if *p > 0 {
                *p = 0;
            } else {
                *p = 144;
            }
        }
        
    }
}
