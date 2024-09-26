// use std::env;
use std::fs;

mod subscription;

fn main() {
    let content = fs::read_to_string("clash_subscription.yml").expect("failed to read cache");
    let proxies: subscription::clash::Config =
        serde_yaml::from_str(&content).expect("failed to parse config");
}
