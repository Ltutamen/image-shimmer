use std::fs;
use macroquad::prelude::*;
use image::ImageFormat::Png;
use crate::configuration::Config;
use crate::image_processing::DimmerApplicationState;

mod configuration;
mod image_processing;

#[macroquad::main(win_config)]
async fn main() {
    let mut application = Application::new();

    loop {
        application.run_frame();
        application.switch();
        next_frame().await
    }
}

fn win_config() -> Conf {
    let img_filepath= configuration::get_config().unwrap().filepath;
    let img = image::io::Reader::open(img_filepath).unwrap();
    let dimensions = img.into_dimensions().unwrap();

    Conf {
        window_width: dimensions.0 as i32,
        window_height: dimensions.1 as i32,
        window_resizable: false,
        ..Default::default()
    }
}

struct Application {
    configuration: Config,
    texture: Texture2D,
    material : Material,
    state: DimmerApplicationState,

}

impl Application {
    fn new() -> Application {
        let configuration = configuration::get_config().unwrap();
        let texture_bytes = image_processing::image_into_bytes(&configuration);
        let texture = Texture2D::from_file_with_format(texture_bytes.as_slice(), Option::Some(Png));
        let material_params: MaterialParams = MaterialParams{
            pipeline_params: Default::default(),
            uniforms: vec![],
            textures: vec![("image".to_string()), ("stripes".to_string())]
        };

        let material = load_material(
            fs::read_to_string("src/shaders/vertex.vs").expect("Vertex shader file not found!").as_str(),
            fs::read_to_string("src/shaders/fragment.fs").expect("Fragment shader file not found!").as_str(),
            material_params).unwrap();
        let state = DimmerApplicationState::new(texture.width() as usize);

        return Application{
            configuration, 
            texture,
            material,
            state,
        }
    }

    fn run_frame(&self) {
        let material = &self.material;
        let texture = &self.texture;
        let bit_state = &self.state.bit_state;
        clear_background(BLACK);

        {
            let wight = texture.width() as usize;
            material.set_texture("image", *texture);
            material.set_texture("stripes",
                                 Texture2D::from_rgba8(wight as u16, 1, &bit_state[0..wight * 4])); //  todo slice from vector
        }

        gl_use_material(*material);

        draw_texture_ex(*texture, -1., -1., WHITE, Default::default());

        gl_use_default_material();

    }

    fn switch(&mut self) {
        let state = &mut self.state.bit_state;
        let fun = self.state.transform;
        fun(state)
    }
}
