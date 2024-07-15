use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngeniaDictionary {
    header: Header,
    body: Body,
    drive_image: DriveImage,
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
    errors: Errors,
}

#[derive(Debug, Deserialize, Serialize)]
struct Device {
    family: String,
    #[serde(rename = "firmwareVersion")]
    firmware_version: String,
    #[serde(rename = "ProductCode")]
    product_code: String,
    #[serde(rename = "RevisionNumber")]
    revision_number: String,
    #[serde(rename = "Interface")]
    interface: String,
    name: String,
    #[serde(rename = "Categories")]
    categories: Categories,
    #[serde(rename = "Registers")]
    registers: Registers,
}

#[derive(Debug, Deserialize, Serialize)]
struct Categories {
    #[serde(rename = "Category")]
    category: Vec<Category>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    id: String,
    #[serde(rename = "Labels")]
    labels: Labels,
}

#[derive(Debug, Deserialize, Serialize)]
struct Labels {
    #[serde(rename = "Label")]
    label: Vec<Label>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Label {
    lang: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Registers {
    #[serde(rename = "Register")]
    register: Vec<Register>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Register {
    access: String,
    address_type: String,
    address: String,
    dtype: String,
    id: String,
    units: String,
    subnode: String,
    cyclic: String,
    desc: String,
    cat_id: String,
    #[serde(rename = "Labels")]
    labels: Labels,
}

#[derive(Debug, Deserialize, Serialize)]
struct Errors {
    #[serde(rename = "Error")]
    error: Vec<Error>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Error {
    id: String,
    affected_module: String,
    error_type: String,
    #[serde(rename = "Labels")]
    labels: Labels,
}

#[derive(Debug, Deserialize, Serialize)]
struct DriveImage {
    encoding: String,
}

pub fn xdf_from_file(contents: &str) -> IngeniaDictionary {
    from_str(&contents).unwrap()
}

impl IngeniaDictionary {
    pub fn data_type(&self, address: u16) -> &str {
        for register in self.body.device.registers.register.iter() {
            let add_string: String = register.address.parse().unwrap();
            let without_prefix = add_string.trim_start_matches("0x");
            let add = match u16::from_str_radix(without_prefix, 16) {
                Ok(value) => value,
                _ => 0u16,
            };
            if add == address {
                return &register.dtype;
            }
        }
        &self.header.version
    }

    pub fn get_reg_uid(&self, address: u16) -> Result<&str, u16> {
        for register in self.body.device.registers.register.iter() {
            let add_string: String = register.address.parse().unwrap();
            let without_prefix = add_string.trim_start_matches("0x");
            let add = match u16::from_str_radix(without_prefix, 16) {
                Ok(value) => value,
                _ => 0u16,
            };
            if add == address {
                return Ok(&register.id);
            }
        }
        Err(0u16)
    }

    pub fn get_product_code(&self) -> u32 {
        self.body.device.product_code.parse().unwrap()
    }

    pub fn get_revision_number(&self) -> u32 {
        self.body.device.revision_number.parse().unwrap()
    }
}
