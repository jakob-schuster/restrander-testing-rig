use crate::{restrander::make_input_filename, config::RestranderConfig};
use config::ProgramResult;
use itertools::iproduct;

mod constants;
mod json;
mod restrander;
mod paf;
mod fastq;
mod config;

fn main() {
    // let error_rates = vec![0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35];
    let mut error_rates: Vec<f64> = vec![];
    for i in 0..10 {
        error_rates.push((0.5 / 10 as f64) * (i as f64));
    }

    let inputs = vec![
        Input::new("aligned_reads.fq.gz", "aligned.paf"),
    ];

    print_results(
        restrander_grid_test(inputs, restrander_generate_configs(error_rates)));
}

#[derive(Clone)]
struct Input {
    fastq: String,
    paf: String,
}

impl Input {
    pub fn new(fastq: &str, paf: &str) -> Input {
        Input {fastq: make_input_filename(&fastq.to_string()), paf: make_input_filename(&paf.to_string())}
    }
}

fn restrander_generate_configs(error_rates: Vec<f64>) -> Vec<RestranderConfig> {
    // make all the config files
    let config_filenames = json::make_configs(error_rates.clone());

    // compile it all together correctly
    error_rates.iter()
        .zip(config_filenames)
        .map(|(error_rate, config_filename)| RestranderConfig{config_filename: config_filename, error_rate: *error_rate})
        .collect()
}

fn restrander_grid_test(inputs: Vec<Input>, configs: Vec<config::RestranderConfig>) -> Vec<config::ProgramResult> {
    // run restrander on the product of inputs and configs
    iproduct!(inputs, configs)
        .map(|(input, config)| (config::GenericProgramConfig{input: input.fastq.clone(), output: constants::OUTPUT_FILENAME.to_string()}, config, paf::parse(input.paf.clone())))
        .map(|(generic_config, restrander_config, paf_reads)| 
            restrander::accuracy_timed_run_config(generic_config, restrander_config, &paf_reads))
        .collect()
}

fn print_results(results: Vec<ProgramResult>) {
    // print CSV header line
    println!("config,correct_percent,incorrect_percent,ambiguous_percent,time_secs");
    
    // print each result
    results.iter()
       .for_each(|result| println!("{},{},{},{},{}", result.config, result.accuracy.correct, result.accuracy.incorrect, result.accuracy.ambiguous, result.duration));
}

/*
fn pychopper_grid_test(inputs: Vec<Input>, configs: Vec<config::PychopperConfig>) {
    iproduct!(inputs, configs)
        .map(|(input, config)| (config::GenericProgramConfig{input: input.fastq.clone(), output: constants::OUTPUT_FILENAME.to_string()}, config, paf::parse(input.paf.clone())))
        .map(|(generic_config, pychopper_config, paf_reads)|
            pychopper::accuracy_timed_run_config(generic_config, pychopper_config, &paf_reads))
}
*/