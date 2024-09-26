use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub proxies: Vec<Proxy>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Proxy {
    #[serde(rename = "vmess")]
    Vmess(Vmess),
}

// https://github.com/MetaCubeX/mihomo/blob/Meta/adapter/outbound/vmess.go#L107-L203
#[derive(Deserialize)]
pub enum Network {
    #[serde(rename = "ws")]
    Ws,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "grpc")]
    Grpc,
}

// https://github.com/MetaCubeX/mihomo/blob/Meta/adapter/outbound/vmess.go#L430-L435
#[derive(Deserialize)]
pub enum PacketEncoding {
    #[serde(rename = "PacketAddr", alias = "Packet")]
    PacketAddr,
    #[serde(rename = "xudp")]
    Xudp,
}

// https://github.com/MetaCubeX/mihomo/blob/Meta/transport/vmess/vmess.go#L93-L104
#[derive(Deserialize)]
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

// https://github.com/MetaCubeX/mihomo/blob/Meta/adapter/outbound/vmess.go#L46-L72
#[derive(Deserialize)]
pub struct Vmess {
    pub name: String,
    pub server: String,
    pub port: u16,

    pub uuid: String,
    #[serde(rename = "alterId")]
    pub alter_id: u8,
    /// encryption method
    pub cipher: Cipher,
    #[serde(rename = "packet-encoding")]
    pub packet_encoding: Option<PacketEncoding>,
    #[serde(rename = "global-padding")]
    pub global_padding: Option<bool>,
    #[serde(rename = "authenticated-length")]
    pub authenticated_length: Option<bool>,
    pub network: Option<Network>,
}
