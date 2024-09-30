use serde::{Deserialize, Serialize};

use crate::subscription::mihomo;

pub mod vmess;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub outbounds: Vec<Outbounds>,
}

impl From<mihomo::Config> for Config {
    fn from(config: mihomo::Config) -> Config {
        Config {
            outbounds: config
                .proxies
                .iter()
                .map(|proxy| (*proxy).clone().into())
                .collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Outbounds {
    #[serde(rename = "vmess")]
    Vmess(vmess::Vmess),
}

impl From<mihomo::Proxy> for Outbounds {
    fn from(proxy: mihomo::Proxy) -> Outbounds {
        match proxy {
            mihomo::Proxy::Vmess(vmess) => Outbounds::Vmess(vmess.into()),
        }
    }
}
