use std::{time::Instant, process::Command, fs::remove_file};

use crate::{config::{GenericProgramConfig, SpecificProgramConfig, ProgramResult, ProgramConfig, PychopperConfig}, constants, fastq, paf::PafRead};

pub fn accuracy_timed_run_config(
    generic_config: GenericProgramConfig, 
    specific_config: SpecificProgramConfig,
    paf_reads: &Vec<PafRead>
) -> ProgramResult {
    println!("doing accuracy timed run");
    // get the backend argument string
    let backend_string = match specific_config.clone() {
        SpecificProgramConfig::Restrander(_) => panic!("aaa"),
        SpecificProgramConfig::Pychopper(PychopperConfig { backend }) => match backend {
            crate::config::PychopperBackend::MachineLearning => "phmm",
            crate::config::PychopperBackend::Edlib => "edlib"
        }
    };

    // run it and time it
    let start = Instant::now();
    let command = format!("{} run pychopper {} {} {}", constants::CONDA_PATH, backend_string, generic_config.clone().input, generic_config.clone().output);
    Command::new(constants::CONDA_PATH)
            .arg("run")
            .arg("pychopper")
            .args(&["-m", backend_string])
            .args(&["-k", "PCS111"])
            .arg(generic_config.clone().input)
            .arg(generic_config.clone().output)
    // Command::new(Con)
            .spawn()
            .expect("pychopper failed to start")
            .wait();
    let duration = start.elapsed().as_secs();

    // determine the accuracy
    let accuracy = fastq::parse_pychopper(generic_config.clone().output, paf_reads);

    // delete the file if necessary
    // remove_file(generic_config.clone().output)
    //     .expect("Couldn't delete file!");

    // construct a program result at the end
    ProgramResult { 
        config: ProgramConfig {
            generic: generic_config,
            specific: specific_config
        },
        duration,
        accuracy
    }
}