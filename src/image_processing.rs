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
    //  pub bit_state: Texture2D,
    pub bit_state: [u8; Config::MAX_IMG_WIGHT * 4],
    pub transform: fn(&mut [u8; Config::MAX_IMG_WIGHT * 4]) -> [u8; Config::MAX_IMG_WIGHT * 4],
}

impl DimmerApplicationState {
    
    pub fn new() -> DimmerApplicationState {

        let bytes = &mut[0 as u8; Config::MAX_IMG_WIGHT * 4];

        //  todo write start state
        for i in 0..Config::MAX_IMG_WIGHT {
            if i % 2 == 0 {
                bytes[i * 4] = 177;
            }
        }

        DimmerApplicationState {
            //  bit_state: (Texture2D::from_rgba8(1280, 1, bytes))
            bit_state: *bytes,
            transform: (|bytes | {
                bytes.clone()
            })
        }
    }

    pub fn new_with(
        init_state : Vec<bool>,
        func: fn(&mut [u8; Config::MAX_IMG_WIGHT * 4]) -> [u8; Config::MAX_IMG_WIGHT * 4]
    ) {
        let bytes = &mut[0 as u8; Config::MAX_IMG_WIGHT * 4];

        for (i, val) in init_state.iter().enumerate() {
            bytes[i] = if *val {
                144
            } else { 0 } as u8;

        }
    }

    pub fn switch(&mut self) {
        let func = self.transform;
        let new_state = func(&mut self.bit_state);
        self.bit_state = new_state;
    }
}
