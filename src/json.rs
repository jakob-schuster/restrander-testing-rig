use std::{fs::File, path::Path, io::Write};

use serde_json::{Value, json};

use crate::constants;

// struct Identifier {
//     input_filename: String,
//     error_rate: f64,
// }

// struct Value {
//     correct_rate: f64,
//     unknown_rate: f64,
//     incorrect_rate: f64,
// }

// pub struct Data {
//     id: Identifier,
//     val: Value,
// }

fn default_config(error_rate: f64) -> Value {
    json!({
        "name": "PCB109",
        "description": "The default configuration. First applies PolyA/PolyT classification, then looks for the standard TSO (SSP) and RTP (VNP) used in PCB109 chemistry.",
        "pipeline": [
            {
                "type": "poly",
                "tail-length": 12,
                "search-size": 200
            },
            {
                "type": "primer",
                "tso": "TTTCTGTTGGTGCTGATATTGCTGGG",
                "rtp": "ACTTGCCTGTCGCTCTATCTTCTTTTTTTTTT",
                "report-artefacts": true
            }
        ],
        "silent": false,
        "exclude-unknowns": false,
        "error-rate": error_rate
    })
}

pub fn make_configs(error_rates: Vec<f64>) -> Vec<String> {
    let base_filepath: &Path = Path::new(constants::CONFIG_PATH);
    
    // construct a bunch of json bodies
    let jsons = error_rates.iter()
        .map(|e| -> Value {default_config(*e)})
        .map(|v| v.to_string());

    let filenames: Vec<String> = error_rates.iter()
        .map(|e| -> String {format!("{}.json", e.to_string()).to_string()})
        .map(|s| -> String {Path::join(base_filepath, Path::new(&s)).to_str().expect("Couldn't make path!").to_string()})
        .collect();

    // create the files themselves
    let files = filenames.iter()
        .map(|s| -> File {File::create(s).expect("Failed to create file!")});

    // write the json bodies into the appropriate files
    let success = files
        .zip(jsons)
        .map(|(mut f, s)| -> Result<usize, std::io::Error> {f.write(s.as_bytes())})
        .all(|r| -> bool {r.is_ok()});

    if !success {
        panic!("Failed to write a json!")
    }

    filenames
}