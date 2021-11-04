use std::fs;
use std::ops::Add;
use macroquad::prelude::*;
use image::ImageFormat::Png;
use std::time::Duration;
use crate::configuration::WinConfig;
use crate::image_processing::{DimmerApplicationState, Vec4f};
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
        let state = DimmerApplicationState::new(texture.width() as usize, shimmer_config);

        return Application{
            configuration,
            texture,
            material,
            state,
        }
    }

    fn run_frame(&mut self) {
        let material = &self.material;
        let texture = &self.texture;
        let image_wight = texture.width() as u16;
        clear_background(BLACK);

        {
            material.set_texture("image", *texture);
            material.set_texture("stripes",
                                 Texture2D::from_rgba8(image_wight, 1, &Application::convert_to_bytes(&self.state.bit_state)));
        }

        gl_use_material(*material);

        draw_texture_ex(*texture, -1., -1., WHITE, Default::default());

        gl_use_default_material();

    }

    fn convert_to_bytes(colors : &Vec<Vec4f>) -> Vec<u8> {
        let byte_vec_size = colors.len() * 4;
        let mut byte_vec = Vec::<u8>::with_capacity(byte_vec_size);


        Application::push_vec_vec4f(colors, &mut byte_vec);

        byte_vec
    }

    fn push_vec_vec4f<'vec>(colors : &Vec<Vec4f>, destination: &'vec mut Vec::<u8>) -> &'vec mut Vec::<u8> {
        for color in colors {
            Application::push_vec4f(color, destination);
        }

        destination
    }

    fn push_vec4f<'vec>(vec : & Vec4f, destination: &'vec mut Vec::<u8>) -> &'vec mut Vec::<u8> {
        Application::push_float(vec.x, destination);
        Application::push_float(vec.y, destination);
        Application::push_float(vec.z, destination);
        Application::push_float(vec.a, destination);

        destination
    }

    //  float[0. -> 1.] -> u8[0 -> 255]
    fn push_float(val: f32, destination: &mut Vec::<u8>) -> &mut Vec::<u8> {
        let byte = (val * 255.) as u8;
        destination.push(byte);

        destination
    }

    fn switch(&mut self) {
        let state = &mut self.state.bit_state;
        let fun = self.state.transform;
        fun(state)
    }
}
