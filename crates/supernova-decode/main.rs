use supernova_decode::config::SupernovaDecodeConfig;

fn main() {
    let _ = SupernovaDecodeConfig::save_defaults_if_not_exists();
}
