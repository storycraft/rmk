#![cfg_attr(not(feature = "build"), no_std)]

#[derive(Debug, Clone)]
pub struct Configuration<'a> {
    pub descriptor: Descriptor<'a>,
    pub hid: Hid,
}

#[derive(Debug, Clone)]
pub struct Descriptor<'a> {
    pub vid: u16,
    pub pid: u16,

    pub manufacturer: Option<&'a str>,
    pub product: Option<&'a str>,
    pub version: Option<u16>,
    pub serial_number: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct Hid {
    pub poll_ms: u8,
    pub nkro: bool,
}

#[cfg(feature = "build")]
pub fn build_config<'a, De: serde::Deserializer<'a>>(de: De) -> Result<String, De::Error> {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct ConfigFile {
        vid: u16,
        pid: u16,

        manufacturer: Option<String>,
        product: Option<String>,
        version: Option<u16>,
        serial_number: Option<String>,
        poll_ms: u8,
        nkro: bool,
    }

    let root: &str = env!("CARGO_CRATE_NAME");

    let ConfigFile {
        vid,
        pid,
        manufacturer,
        product,
        version,
        serial_number,
        poll_ms,
        nkro,
    } = ConfigFile::deserialize(de)?;

    Ok(std::format!(
        "::{root}::Configuration::<'static> {{
            descriptor: ::{root}::Descriptor {{
                vid: {vid},
                pid: {pid},
                manufacturer: {manufacturer:?},
                product: {product:?},
                version: {version:?},
                serial_number: {serial_number:?},
            }},
            hid: ::{root}::Hid {{
                poll_ms: {poll_ms},
                nkro: {nkro},
            }},
        }}"
    ))
}
