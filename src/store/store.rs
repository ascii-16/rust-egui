use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub value: f64,
    pub category: String,
    pub from_unit: String,
    pub to_unit: String,
}

pub fn save_conversion(conversion: Conversion) -> std::io::Result<()> {
    let mut conversions: Vec<Conversion> =
        if let Ok(data) = fs::read_to_string("data/conversions.json") {
            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

    conversions.push(conversion);

    let json = serde_json::to_string_pretty(&conversions)?;
    fs::write("data/conversions.json", json)?;
    Ok(())
}

pub fn load_conversion() -> Vec<Conversion> {
    let data =
        fs::read_to_string("data/conversions.json").expect("Unable to read conversions.json");
    println!("{}", data);

    let conversions: Vec<Conversion> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    conversions
}
