use supernova_macros::SupernovaConfig;
use serde::{Deserialize, Serialize};

#[derive(SupernovaConfig, Serialize, Deserialize, Debug)]
#[supernova_config(name = "test")]
#[serde(default)]
struct TestConfig {
    test: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self { test: "test-hey".to_string() }
    }
}

fn main() {
    let _ = TestConfig::save_defaults_if_not_exists();
}
