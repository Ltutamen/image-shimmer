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

pub fn get_config() -> (WinConfig, ShimmerConfig) {
    let config_file = config::File::with_name(SETTINGS_FILE);

    let mut config = config::Config::default();

    let conf = config.merge(config_file).unwrap();
    validate_config(conf)
}

pub fn validate_config(config: &config::Config) -> (WinConfig, ShimmerConfig) {

    let img_path = config.get_str(IMAGE_FILE_KEY);
    let slices_count = config.get_int(SLICES_COUNT_KEY);
    let shimmer_freq = config.get_int(SHIMMER_FREQ_KEY);

    let shimmer_config_map = &config.get_table(SHIMMER_CONFIG_FREQ_KEY).unwrap();
    let shimmer_config_type = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_TYPE_KEY).unwrap().clone().into_str().unwrap();
    let shimmer_config_config = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_CONFIG_KEY).unwrap().clone();

    let shimmer_config_config_ = shimmer_config_config.into_table().unwrap();

    return ( WinConfig {
        filepath: img_path.unwrap(),
        slices_cont: slices_count.unwrap() as i32,
        dimm_freq: shimmer_freq.unwrap() as i32,
        scale: 1.0, },
    ShimmerConfig {
            shimmer_type: ShimmerType::from_str(shimmer_config_type.as_str()).unwrap(),
            config: shimmer_config_config_,
        },)
}

pub struct WinConfig {
    pub filepath: String,
    pub slices_cont: i32,
    pub dimm_freq: i32,
    pub scale: f32,
}

pub enum ShimmerType {
    NMStripe,
}

impl FromStr for ShimmerType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NMStripe" => Ok(ShimmerType::NMStripe),
            _ => Err(()),
        }
    }
}

pub struct ShimmerConfig {
    pub shimmer_type: ShimmerType,
    pub config: HashMap<String, Value>,
}