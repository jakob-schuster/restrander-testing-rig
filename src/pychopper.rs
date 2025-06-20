use std::{collections::HashMap, fs::remove_file, process::Command, time::Instant};

use crate::{
    config::{
        GenericProgramConfig, ProgramConfig, ProgramResult, PychopperConfig, SpecificProgramConfig,
    },
    constants, fastq,
    paf::{PafRead, PafReads},
};

pub fn accuracy_timed_run_config(
    generic_config: &GenericProgramConfig,
    specific_config: &SpecificProgramConfig,
    paf_reads: &PafReads,
) -> ProgramResult {
    // get the backend argument string
    let backend_string = match specific_config.clone() {
        SpecificProgramConfig::Restrander(_) => panic!("aaa"),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend,
            protocol: _,
        }) => match backend {
            crate::config::PychopperBackend::MachineLearning => "phmm",
            crate::config::PychopperBackend::Edlib => "edlib",
        },
    };

    let protocol_string = match specific_config.clone() {
        SpecificProgramConfig::Restrander(_) => panic!("wrong config provided!"),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: _,
            protocol,
        }) => match protocol {
            crate::config::Protocol::PCB109 => "PCS109",
            crate::config::Protocol::PCB111 => "PCS111",
        },
    };

    // run it and time it
    let start = Instant::now();
    let command = format!(
        "{} run pychopper {} {} {}",
        constants::CONDA_PATH,
        backend_string,
        generic_config.clone().input,
        generic_config.clone().output
    );
    Command::new(constants::CONDA_PATH)
        .arg("run")
        .arg("pychopper")
        .args(&["-m", backend_string])
        .args(&["-k", protocol_string])
        .arg(generic_config.clone().input)
        .arg(generic_config.clone().output)
        // Command::new(Con)
        .spawn()
        .expect("pychopper failed to start")
        .wait()
        .expect("pychopper failed to terminate");
    let duration = start.elapsed().as_secs();

    // determine the accuracy
    let accuracy = fastq::parse(&generic_config.clone().output, &paf_reads, true);

    // delete the file if necessary
    // remove_file(generic_config.clone().output)
    //     .expect("Couldn't delete file!");

    // construct a program result at the end
    ProgramResult {
        config: ProgramConfig {
            generic: generic_config.clone(),
            specific: specific_config.clone(),
        },
        duration,
        accuracy,
    }
}

pub fn run(
    generic_config: &GenericProgramConfig,
    specific_config: &SpecificProgramConfig,
    paf_reads: &PafReads,
) {
    // get the backend argument string
    let backend_string = match specific_config.clone() {
        SpecificProgramConfig::Restrander(_) => panic!("aaa"),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend,
            protocol: _,
        }) => match backend {
            crate::config::PychopperBackend::MachineLearning => "phmm",
            crate::config::PychopperBackend::Edlib => "edlib",
        },
    };

    let protocol_string = match specific_config.clone() {
        SpecificProgramConfig::Restrander(_) => panic!("wrong config provided!"),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: _,
            protocol,
        }) => match protocol {
            crate::config::Protocol::PCB109 => "PCS109",
            crate::config::Protocol::PCB111 => "PCS111",
        },
    };

    Command::new(constants::CONDA_PATH)
        .arg("run")
        .arg("pychopper")
        .args(&["-m", backend_string])
        .args(&["-k", protocol_string])
        .arg(generic_config.clone().input)
        .arg(generic_config.clone().output)
        .spawn()
        .expect("pychopper failed to start")
        .wait()
        .expect("pychopper failed to terminate");
}
