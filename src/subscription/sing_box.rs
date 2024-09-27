use serde::Deserialize;

use crate::subscription::mihomo;

#[derive(Deserialize)]
pub struct Config {
    pub outbounds: Vec<Outbounds>,
}

impl From<mihomo::Config> for Config {
    fn from(config: mihomo::Config) -> Config {
        Config {
            outbounds: config.proxies.iter().map(|proxy| (*proxy).clone().into()).collect(),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Outbounds {
    #[serde(rename = "vmess")]
    Vmess(Vmess),
}

impl From<mihomo::Proxy> for Outbounds {
    fn from(proxy: mihomo::Proxy) -> Outbounds {
        match proxy {
            mihomo::Proxy::Vmess(vmess) => Outbounds::Vmess(vmess.into()),
        }
    }
}

// https://github.com/SagerNet/sing-box/blob/dev-next/option/types.go#L103
#[derive(Deserialize, Clone)]
pub enum Network {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

// https://github.com/SagerNet/sing-box/blob/dev-next/outbound/vmess.go#L71-L76
#[derive(Deserialize, Clone)]
pub enum PacketEncoding {
    #[serde(rename = "packetaddr")]
    PacketAddr,
    #[serde(rename = "xudp")]
    Xudp,
}

impl From<mihomo::PacketEncoding> for PacketEncoding {
    fn from(packet_encoding: mihomo::PacketEncoding) -> PacketEncoding {
        match packet_encoding {
            mihomo::PacketEncoding::PacketAddr => PacketEncoding::PacketAddr,
            mihomo::PacketEncoding::Xudp => PacketEncoding::Xudp,
        }
    }
}

// https://github.com/SagerNet/sing-vmess/blob/dev/client.go#L43-L53
#[derive(Deserialize, Clone)]
pub enum Security {
    #[serde(rename = "aes-128-gcm")]
    Aes128Gcm,
    #[serde(rename = "aes-128-cfb")]
    Aes128Cfb,
    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,
    #[serde(rename = "none", alias = "zero")]
    None,
    #[serde(rename = "auto")]
    Auto,
}

impl From<mihomo::Cipher> for Security {
    fn from(cipher: mihomo::Cipher) -> Security {
        match cipher {
            mihomo::Cipher::Aes128Gcm => Security::Aes128Gcm,
            mihomo::Cipher::Chacha20Poly1305 => Security::Chacha20Poly1305,
            mihomo::Cipher::None => Security::None,
            mihomo::Cipher::Auto => Security::Auto,
        }
    }
}

// https://github.com/SagerNet/sing-box/blob/dev-next/option/outbound.go#L141-L144
// https://github.com/SagerNet/sing-box/blob/dev-next/option/vmess.go#L17-L30
#[derive(Deserialize, Clone)]
pub struct Vmess {
    pub name: String,
    pub server: String,
    pub server_port: u16,

    pub uuid: String,
    pub security: Option<Security>,
    pub alter_id: Option<i32>,
    pub global_padding: Option<bool>,
    pub authenticated_length: Option<bool>,
    pub network: Option<Network>,
    pub packet_encoding: Option<PacketEncoding>,
}

impl From<mihomo::Vmess> for Vmess {
    fn from(vmess: mihomo::Vmess) -> Vmess {
        Vmess {
            name: vmess.name,
            server: vmess.server,
            server_port: vmess.port,

            uuid: vmess.uuid,
            security: Some(vmess.cipher.into()),
            alter_id: Some(vmess.alter_id),
            global_padding: vmess.global_padding,
            authenticated_length: vmess.authenticated_length,
            network: if vmess.udp.unwrap_or(false) {
                None
            } else {
                Some(Network::Tcp)
            },
            packet_encoding: vmess
                .packet_encoding
                .map(|packet_encoding| packet_encoding.into()),
        }
    }
}
