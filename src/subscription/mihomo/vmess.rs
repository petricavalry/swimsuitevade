use crate::subscription::sing_box;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum PacketEncoding {
    #[serde(rename = "PacketAddr", alias = "Packet")]
    PacketAddr,
    #[serde(rename = "xudp")]
    Xudp,
}

impl From<sing_box::vmess::PacketEncoding> for PacketEncoding {
    fn from(packet_encoding: sing_box::vmess::PacketEncoding) -> PacketEncoding {
        match packet_encoding {
            sing_box::vmess::PacketEncoding::PacketAddr => PacketEncoding::PacketAddr,
            sing_box::vmess::PacketEncoding::Xudp => PacketEncoding::Xudp,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Security {
    #[serde(rename = "aes-128-gcm")]
    Aes128Gcm,
    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
}

impl From<sing_box::vmess::Security> for Security {
    fn from(security: sing_box::vmess::Security) -> Security {
        match security {
            sing_box::vmess::Security::Aes128Gcm => Security::Aes128Gcm,
            sing_box::vmess::Security::Chacha20Poly1305 => Security::Chacha20Poly1305,
            sing_box::vmess::Security::None => Security::None,
            sing_box::vmess::Security::Auto => Security::Auto,
            _ => panic!("unknown security"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Vmess {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub udp: Option<bool>,

    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: i32,
    /// encryption method
    #[serde(rename = "cipher")]
    pub security: Security,
    #[serde(rename = "packet-encoding")]
    pub packet_encoding: Option<PacketEncoding>,
    #[serde(rename = "global-padding")]
    pub global_padding: Option<bool>,
    #[serde(rename = "authenticated-length")]
    pub authenticated_length: Option<bool>,
}

impl From<sing_box::vmess::Vmess> for Vmess {
    fn from(vmess: sing_box::vmess::Vmess) -> Vmess {
        Vmess {
            name: vmess.name,
            server: vmess.server,
            port: vmess.server_port,

            uuid: vmess.uuid,
            security: vmess.security.unwrap().into(), // TODO ^ default security
            alter_id: vmess.alter_id.unwrap(),        // TODO ^ default alter id
            global_padding: vmess.global_padding,
            authenticated_length: vmess.authenticated_length,
            udp: Some(match vmess.network {
                Some(enable) => match enable {
                    sing_box::vmess::Network::Tcp => false,
                    sing_box::vmess::Network::Udp => panic!("mihomo doesn't support udp only"),
                },
                None => true,
            }),
            packet_encoding: vmess
                .packet_encoding
                .map(|packet_encoding| packet_encoding.into()),
        }
    }
}
