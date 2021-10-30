use std::fs;
use std::ops::Add;
use macroquad::prelude::*;
use image::ImageFormat::Png;
use std::time::Duration;
use crate::configuration::WinConfig;
use crate::image_processing::DimmerApplicationState;
use std::time::SystemTime;

mod configuration;
mod image_processing;

#[macroquad::main(win_config)]
async fn main() {
    let mut application = Application::new();
    let frame_time = application.configuration.frame_time as u64;

    loop {
        let frame_end_time = SystemTime::now().add(Duration::from_millis(frame_time));
        application.run_frame();
        application.switch();
        let skip = frame_end_time.duration_since(SystemTime::now())
            .unwrap();

        println!("AAA:{:?}", skip);
        std::thread::sleep(skip);
        next_frame().await
    }
}

fn win_config() -> Conf {
    let img_filepath = configuration::get_config().0.filepath;
    let img = image::io::Reader::open(img_filepath).unwrap();
    let dimensions = img.into_dimensions().unwrap();

    Conf {
        window_width: dimensions.0 as i32,
        window_height: dimensions.1 as i32,
        window_resizable: false,
        sample_count: 0,
        ..Default::default()
    }
}

struct Application {
    configuration: WinConfig,
    texture: Texture2D,
    material : Material,
    state: DimmerApplicationState,

}

impl Application {
    fn new() -> Application {
        let (configuration, shimmer_config) = configuration::get_config();
        let texture_bytes = image_processing::image_into_bytes(&configuration.filepath);
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
        let state = DimmerApplicationState::from_config(texture.width() as usize, shimmer_config);

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
                                 Texture2D::from_rgba8(wight as u16, 1, &bit_state[0..wight * 4]));
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
