use crate::subscription::sing_box;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub proxies: Vec<Proxy>,
}

impl From<sing_box::Config> for Config {
    fn from(config: sing_box::Config) -> Config {
        Config {
            proxies: config.outbounds.iter().map(|proxy| (*proxy).clone().into()).collect()
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Proxy {
    #[serde(rename = "vmess")]
    Vmess(Vmess),
}

impl From<sing_box::Outbounds> for Proxy {
    fn from(outbounds: sing_box::Outbounds) -> Proxy {
        match outbounds {
            sing_box::Outbounds::Vmess(vmess) => Proxy::Vmess(vmess.into())
        }
    }
}

// https://github.com/MetaCubeX/mihomo/blob/Meta/adapter/outbound/vmess.go#L430-L435
#[derive(Deserialize, Clone)]
pub enum PacketEncoding {
    #[serde(rename = "PacketAddr", alias = "Packet")]
    PacketAddr,
    #[serde(rename = "xudp")]
    Xudp,
}

impl From<sing_box::PacketEncoding> for PacketEncoding {
    fn from(packet_encoding: sing_box::PacketEncoding) -> PacketEncoding {
        match packet_encoding {
            sing_box::PacketEncoding::PacketAddr => PacketEncoding::PacketAddr,
            sing_box::PacketEncoding::Xudp => PacketEncoding::Xudp,
        }
    }
}

// https://github.com/MetaCubeX/mihomo/blob/Meta/transport/vmess/vmess.go#L93-L104
#[derive(Deserialize, Clone)]
pub enum Cipher {
    #[serde(rename = "aes-128-gcm")]
    Aes128Gcm,
    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
}

impl From<sing_box::Security> for Cipher {
    fn from(security: sing_box::Security) -> Cipher {
        match security {
            sing_box::Security::Aes128Gcm => Cipher::Aes128Gcm,
            sing_box::Security::Chacha20Poly1305 => Cipher::Chacha20Poly1305,
            sing_box::Security::None => Cipher::None,
            sing_box::Security::Auto => Cipher::Auto,
            _ => panic!("unknown security"),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Vmess {
    pub name: String,
    pub server: String,
    pub port: u16,
    pub udp: Option<bool>,

    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: i32,
    /// encryption method
    pub cipher: Cipher,
    #[serde(rename = "packet-encoding")]
    pub packet_encoding: Option<PacketEncoding>,
    #[serde(rename = "global-padding")]
    pub global_padding: Option<bool>,
    #[serde(rename = "authenticated-length")]
    pub authenticated_length: Option<bool>,
}

impl From<sing_box::Vmess> for Vmess {
    fn from(vmess: sing_box::Vmess) -> Vmess {
        Vmess {
            name: vmess.name,
            server: vmess.server,
            port: vmess.server_port,

            uuid: vmess.uuid,
            cipher: vmess.security.unwrap().into(),  // TODO ^ default security
            alter_id: vmess.alter_id.unwrap(),  // TODO ^ default alter id
            global_padding: vmess.global_padding,
            authenticated_length: vmess.authenticated_length,
            udp: Some(match vmess.network {
                Some(enable) => match enable {
                    sing_box::Network::Tcp => false,
                    sing_box::Network::Udp => panic!("mihomo doesn't support udp only"),
                }
                None => true,
            }),
            packet_encoding: vmess.packet_encoding.map(|packet_encoding| packet_encoding.into()),
        }
    }
}
