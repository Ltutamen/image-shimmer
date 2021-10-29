use std::collections::HashMap;
use std::str::FromStr;

use config::{ConfigError, Value};

static SETTINGS_FILE: &str = "resources/config.json";
static IMAGE_FILE_KEY: &str = "image_path";
static SLICES_COUNT_KEY: &str = "slices_count";
static SHIMMER_FREQ_KEY: &str = "shimmer_freq";

static SHIMMER_CONFIG_FREQ_KEY: &str = "shimmer_config";
static  SHIMMER_CONFIG_SHIMMER_TYPE_KEY: &str = "shimmer_type";
static  SHIMMER_CONFIG_SHIMMER_CONFIG_KEY: &str = "shimmer_config";

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

    let shimmer_config_map = &config.get_table(SHIMMER_CONFIG_FREQ_KEY).unwrap();
    let shimmer_config_type = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_TYPE_KEY).unwrap().clone().into_str().unwrap();
    let shimmer_config_config = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_CONFIG_KEY).unwrap().clone();

    let shimmer_config_config_ = shimmer_config_config.into_table().unwrap();

    return Config{
        filepath: img_path.unwrap(),
        slices_cont: slices_count.unwrap() as i32,
        dimm_freq: shimmer_freq.unwrap() as i32,
        scale: 1.0,
        shimmer_config: ShimmerConfig {
            shimmer_type: ShimmerType::from_str(shimmer_config_type.as_str()).unwrap(),
            config: shimmer_config_config_,
        },
    }
}

pub struct Config {
    pub filepath: String,
    pub slices_cont: i32,
    pub dimm_freq: i32,
    pub scale: f32,
    pub shimmer_config: ShimmerConfig,
}

pub enum ShimmerType {
    NStripe,
}

impl FromStr for ShimmerType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NStripe" => Ok(ShimmerType::NStripe),
            _ => Err(()),
        }
    }
}

pub struct ShimmerConfig {
    shimmer_type: ShimmerType,
    config: HashMap<String, Value>,
}