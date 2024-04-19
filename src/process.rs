use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

/// Product struct
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Product {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Create Timestamp")]
    create_timestamp: String,
    #[serde(rename = "Created By")]
    create_by: String,
    code: String,
    #[serde(rename = "English Name")]
    english_name: String,
    #[serde(rename = "Chinese Name")]
    chinese_name: String,
}

/// Convert CSV to JSON
/// input: &str - input CSV file
/// output: &str - output JSON file
pub fn csv_to_json(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut vec = Vec::with_capacity(256);
    for result in reader.deserialize() {
        let product: Product = result?;
        vec.push(product);
    }
    let json = serde_json::to_string_pretty(&vec)?;
    std::fs::write(output, json)?;
    Ok(())
}
