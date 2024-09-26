// use std::env;
use std::fs;
use swimsuitevade::subscription::mihomo::Config;

fn main() {
    let content = fs::read_to_string("clash_subscription.yml").expect("failed to read cache");
    let _clash_config: Config = serde_yaml::from_str(&content).expect("failed to parse config");
}
