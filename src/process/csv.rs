use crate::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde_json::Value;

/// Convert CSV to JSON
/// input: &str - input CSV file
/// output: &str - output JSON file
pub fn csv_to_json(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut vec = Vec::with_capacity(256);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        vec.push(json_value);
    }

    let result = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&vec)?,
        OutputFormat::Yaml => serde_yaml::to_string(&vec)?,
    };

    let output = format!("{}.{}", output, format);

    std::fs::write(output, result)?;
    Ok(())
}
