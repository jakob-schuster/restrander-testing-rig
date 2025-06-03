use std::collections::HashMap;
use std::fs::remove_file;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use crate::config::{self, ProgramResult};
use crate::json::Config;
use crate::paf::{PafRead, PafReads};
use crate::{constants, fastq};

pub fn _make_output_filename(input_filename: &String, error_rate: f64) -> String {
    format!("{}_{}_restrander_out.fq", input_filename, error_rate)
}

pub fn _timed_run(
    input_filename: &str,
    output_filename: &str,
    config_filename: &str,
) -> (Output, Duration) {
    let start = Instant::now();
    let output = _run(input_filename, output_filename, config_filename);
    let duration = start.elapsed();

    (output, duration)
}

pub fn accuracy_timed_run_config(
    generic_config: &config::GenericProgramConfig,
    specific_config: &str,
    paf_reads: &PafReads,
) -> config::ProgramResult {
    // run it and time it
    let start = Instant::now();
    Command::new(constants::RESTRANDER_PATH)
        .arg(&generic_config.input)
        .arg(&generic_config.output)
        .arg(specific_config)
        .output()
        .expect("restrander failed to start");
    let duration = start.elapsed();

    // determine the accuracy
    let accuracy = fastq::parse(&generic_config.clone().output, paf_reads, false);

    // delete the file if necessary
    // remove_file(generic_config.clone().output).expect("Couldn't delete file!");

    // combine all this together to build the result
    ProgramResult {
        duration: duration.as_secs(),
        config: config::ProgramConfig {
            generic: generic_config.clone(),
            specific: config::SpecificProgramConfig::Restrander(config::RestranderConfig {
                config_filename: specific_config.to_string(),
            }),
        },
        accuracy,
    }
}

pub fn _run(input_filename: &str, output_filename: &str, config_filename: &str) -> Output {
    Command::new(constants::RESTRANDER_PATH)
        .arg(input_filename)
        .arg(output_filename)
        .arg(config_filename)
        .output()
        .expect("restrander failed to start")
}
