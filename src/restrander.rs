use std::fs::remove_file;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use crate::paf::PafRead;
use crate::{constants, paf, fastq};
use crate::config::{self, ProgramResult};
use crate::fastq::AccuracyResultExact;

pub fn make_input_filename(input_filename: &String) -> String {
    format!("{}/{}", constants::DATA_PATH, input_filename)
}

pub fn make_output_filename(input_filename: &String, error_rate: f64) -> String {
    format!("{}_{}_restrander_out.fq", input_filename, error_rate)
}

pub fn timed_run(input_filename: &String, output_filename: &String, config_filename: &String) -> (Output, Duration) {
    let start = Instant::now();
    let output = run(input_filename, output_filename, config_filename);
    let duration = start.elapsed();

    return (output, duration)
}

pub fn accuracy_timed_run_config(generic_config: config::GenericProgramConfig, specific_config: config::RestranderConfig, paf_reads: &Vec<PafRead>) -> config::ProgramResult {
    // run it and time it
    let start = Instant::now();
    Command::new(constants::RESTRANDER_PATH)
            .arg(generic_config.clone().input)
            .arg(generic_config.clone().output)
            .arg(specific_config.clone().config_filename)
            .output()
            .expect("restrander failed to start");
    let duration = start.elapsed();

    // determine the accuracy
    let accuracy = fastq::parse_restrander(generic_config.clone().output, paf_reads);

    // delete the file if necessary
    if true {
        remove_file(generic_config.clone().output).expect("Couldn't delete file!");
    }

    // combine all this together to build the result
    ProgramResult {
        duration: duration.as_secs(),
        config: config::ProgramConfig {
            generic: generic_config, 
            specific: config::SpecificProgramConfig::Restrander(specific_config.clone())
        },
        accuracy: accuracy
    }
}

pub fn run(input_filename: &String, output_filename: &String, config_filename: &String) -> Output {
    Command::new(constants::RESTRANDER_PATH)
            .arg(input_filename)
            .arg(output_filename)
            .arg(config_filename)
            .output()
            .expect("restrander failed to start")
}