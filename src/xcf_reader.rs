use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngeniaDictionary {
    header: Header,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Header {
    version: String,
    default_language: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Body {
    device: Device,
}

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    #[serde(rename = "Interface")]
    interface: String,
    #[serde(rename = "PartNumber")]
    part_number: String,
    #[serde(rename = "ProductCode")]
    product_code: String,
    #[serde(rename = "RevisionNumber")]
    revision_number: String,
    #[serde(rename = "firmwareVersion")]
    firmware_version: String,
    #[serde(rename = "Registers")]
    registers: Registers,
}

#[derive(Debug, Deserialize, Serialize)]
struct Registers {
    #[serde(rename = "Register")]
    register: Vec<Register>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Register {
    access: String,
    dtype: String,
    id: String,
    storage: String,
    subnode: String,
}

pub fn xcf_from_file(contents: &str) -> IngeniaDictionary {
    from_str(&contents).unwrap()
}

impl IngeniaDictionary {
    pub fn data_type(&self, uid: &str) -> &str {
        for register in self.body.device.registers.register.iter() {
            if uid == register.id {
                return &register.dtype;
            }
        }
        &self.header.version
    }

    pub fn get_default(&self, uid: &str) -> &str {
        for register in self.body.device.registers.register.iter() {
            if uid == register.id {
                return &register.storage;
            }
        }
        &self.header.version
    }
}
