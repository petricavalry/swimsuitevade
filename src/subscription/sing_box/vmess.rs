use crate::subscription::mihomo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PacketEncoding {
    #[serde(rename = "packetaddr")]
    PacketAddr,
    #[serde(rename = "xudp")]
    Xudp,
}

impl From<mihomo::vmess::PacketEncoding> for PacketEncoding {
    fn from(packet_encoding: mihomo::vmess::PacketEncoding) -> PacketEncoding {
        match packet_encoding {
            mihomo::vmess::PacketEncoding::PacketAddr => PacketEncoding::PacketAddr,
            mihomo::vmess::PacketEncoding::Xudp => PacketEncoding::Xudp,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

impl From<mihomo::vmess::Security> for Security {
    fn from(cipher: mihomo::vmess::Security) -> Security {
        match cipher {
            mihomo::vmess::Security::Aes128Gcm => Security::Aes128Gcm,
            mihomo::vmess::Security::Chacha20Poly1305 => Security::Chacha20Poly1305,
            mihomo::vmess::Security::None => Security::None,
            mihomo::vmess::Security::Auto => Security::Auto,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
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

impl From<mihomo::vmess::Vmess> for Vmess {
    fn from(vmess: mihomo::vmess::Vmess) -> Vmess {
        Vmess {
            name: vmess.name,
            server: vmess.server,
            server_port: vmess.port,

            uuid: vmess.uuid,
            security: Some(vmess.security.into()),
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
