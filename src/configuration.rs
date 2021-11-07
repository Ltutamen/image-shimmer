use std::collections::HashMap;
use std::str::FromStr;

use config::Value;

static SETTINGS_FILE: &str = "resources/config.json";
static IMAGE_FILE_KEY: &str = "image_path";
static FRAME_TIME_KEY: &str = "frame_time";

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
    let frame_time = config.get_int(FRAME_TIME_KEY);

    let shimmer_config_map = &config.get_table(SHIMMER_CONFIG_FREQ_KEY).unwrap();
    let shimmer_config_type = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_TYPE_KEY).unwrap().clone().into_str().unwrap();
    let shimmer_config_config = shimmer_config_map.get(SHIMMER_CONFIG_SHIMMER_CONFIG_KEY).unwrap().clone();

    let shimmer_config_config_ = shimmer_config_config.into_table().unwrap();

    return ( WinConfig {
        filepath: img_path.unwrap(),
        frame_time: frame_time.unwrap() as i32,
    },
    ShimmerConfig {
            shimmer_type: ShimmerType::from_str(shimmer_config_type.as_str()).unwrap(),
            config: shimmer_config_config_,
        },)
}

pub struct WinConfig {
    pub filepath: String,
    pub frame_time: i32,
}

pub enum ShimmerType {
    NMStripe,
    Array,
}

impl FromStr for ShimmerType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NMStripe" => Ok(ShimmerType::NMStripe),
            "Array" => Ok(ShimmerType::Array),
            e => Err(format!("ShimmerType is on unknown type:{}", e)),
        }
    }
}

pub struct ShimmerConfig {
    pub shimmer_type: ShimmerType,
    pub config: HashMap<String, Value>,
}