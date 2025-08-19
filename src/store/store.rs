use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub value: f64,
    pub category: String,
    pub from_unit: String,
    pub to_unit: String,
}

pub fn save_conversion(conversion: &Conversion) -> std::io::Result<()> {
  let json = serde_json::to_string_pretty(conversion)?;
    fs::write("data/conversions.json", json)?;
    Ok(())
}