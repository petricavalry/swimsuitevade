use serde::de::{Deserializer, SeqAccess, Visitor};

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Config {
    pub proxies: Proxies,
}

#[derive(Debug)]
struct Proxies {
}

impl<'de> Deserialize<'de> for Proxies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProxiesVisitor;

        impl<'de> Visitor<'de> for ProxiesVisitor {
            type Value = Proxies;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("clash proxies")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Proxies, V::Error>
            where
                V: SeqAccess<'de>
            {
                let proxies = seq.next_element::<Proxies>()?;
                println!("{:?}", proxies);
                unimplemented!()
            }
        }

        deserializer.deserialize_seq(ProxiesVisitor)
    }
}
