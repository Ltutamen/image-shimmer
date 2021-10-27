use std::io::Read;
use macroquad::prelude::Vec4;

use crate::configuration::Config;

pub fn image_into_bytes(config: &Config) -> Vec<u8> {

    let image_path = &config.filepath;

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
    
    pub fn new(img_wight: usize) -> DimmerApplicationState {

        //  4 u8 in f32, 4 f32 in color
        let mut bit_state = Vec::<u8>::with_capacity(img_wight * 4 * 4);

        //  todo write start state
        for i in 0..img_wight {
            if i % 2 == 0 {
                bit_state.push(144);
            } else {
                bit_state.push(0);
            }
            bit_state.push(0);
            bit_state.push(0);
            bit_state.push(0);
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

    pub fn switch(&mut self) {
        let func = self.transform;
        let new_state = func(&mut self.bit_state);
    }

    fn set_stripe(&mut self, pos: usize, active: bool) {
        self.bit_state[pos * 16] = 144;
    }
}
