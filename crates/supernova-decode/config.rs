use serde::{Serialize, Deserialize};
use supernova_macros::SupernovaConfig;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DecoderConfig {
    #[serde(rename = "beam")]
    Beam { num_beams: u32 },
    #[serde(rename = "greedy")]
    Greedy
}

#[derive(SupernovaConfig, Serialize, Deserialize)]
#[supernova_config(name = "decode")]
#[serde(default)]
pub struct SupernovaDecodeConfig {
    decoder: DecoderConfig
}

impl Default for SupernovaDecodeConfig {
    fn default() -> Self {
        Self {
            decoder: DecoderConfig::Beam { num_beams: 4 }
        }
    }
}