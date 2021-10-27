use config::ConfigError;

static SETTINGS_FILE: &str = "resources/config.json";
static IMAGE_FILE_KEY: &str = "image_path";
static SLICES_COUNT_KEY: &str = "slices_count";
static SHIMMER_FREQ_KEY: &str = "shimmer_freq";


pub fn get_config() -> Result<Config, ConfigError> {
    let config_file = config::File::with_name(SETTINGS_FILE);

    let mut config = config::Config::default();

    config.merge(config_file)
        .map(|conf |validate_config(conf))
}


pub fn validate_config(config: &config::Config) -> Config {

    let img_path = config.get_str(IMAGE_FILE_KEY);
    let slices_count = config.get_int(SLICES_COUNT_KEY);
    let shimmer_freq = config.get_int(SHIMMER_FREQ_KEY);

    return Config{
        filepath: img_path.unwrap(),
        slices_cont: slices_count.unwrap() as i32,
        dimm_freq: shimmer_freq.unwrap() as i32,
        scale: 1.0}
}

pub struct Config {
    pub filepath: String,
    pub slices_cont: i32,
    pub dimm_freq: i32,
    pub scale: f32,
}

impl Config {
    pub(crate) const MAX_IMG_WIGHT: usize = 5192;
}
