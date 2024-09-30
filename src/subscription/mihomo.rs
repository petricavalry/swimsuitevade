use crate::subscription::sing_box;
use serde::{Deserialize, Serialize};

pub mod vmess;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub proxies: Vec<Proxy>,
}

impl From<sing_box::Config> for Config {
    fn from(config: sing_box::Config) -> Config {
        Config {
            proxies: config
                .outbounds
                .iter()
                .map(|proxy| (*proxy).clone().into())
                .collect(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Proxy {
    #[serde(rename = "vmess")]
    Vmess(vmess::Vmess),
}

impl From<sing_box::Outbounds> for Proxy {
    fn from(outbounds: sing_box::Outbounds) -> Proxy {
        match outbounds {
            sing_box::Outbounds::Vmess(vmess) => Proxy::Vmess(vmess.into()),
        }
    }
}
