use std::fs;
use swimsuitevade::subscription::{mihomo, sing_box};

fn main() {
    let content = fs::read_to_string("clash_subscription.yml").expect("failed to read cache");
    let mihomo_config: mihomo::Config =
        serde_yaml::from_str(&content).expect("failed to parse config");
    let sing_box_config: sing_box::Config = mihomo_config.into();
    let _mihomo_config: mihomo::Config = sing_box_config.into();
}
