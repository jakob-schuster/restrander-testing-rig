use std::{env, fs, process::Command, collections::HashSet};

use config::{ProgramResult, PychopperConfig, SpecificProgramConfig, Protocol, GenericProgramConfig};
use itertools::{iproduct, Itertools};
use paf::PafReads;

mod constants;
mod json;
mod restrander;
mod pychopper;
mod paf;
mod fastq;
mod config;

fn not_main() {
    // collect the input filenames from the command line args
    let input = Input::new_from_args();

    // let restrander_config = get_paths(input.clone().config_dir)
    //     .first()
    //     .expect("No restrander config in that directory!");

    let restrander_config = "../config/error-rate-0.35.json".to_string();
    
    let pychopper_config = SpecificProgramConfig::Pychopper(PychopperConfig {
        backend: config::PychopperBackend::Edlib, 
        protocol: input.clone().protocol
    });

    let paf_reads = paf::parse(input.paf);

    compare(input.fastq, restrander_config, pychopper_config, paf_reads);

    println!("done");
}

fn compare(
    input_fastq: String, 
    restrander_config: String, 
    pychopper_config: SpecificProgramConfig, 
    paf_reads: PafReads
) {
    let output_fastq = "temp.fq".to_string();
    restrander::_run(&input_fastq, &output_fastq, &restrander_config);
    let restrander_categorised_reads = fastq::parse_categorise(
        output_fastq.clone(),
        paf_reads.clone(),
        false
    );

    pychopper::run(
        GenericProgramConfig { 
            input: input_fastq, 
            output: output_fastq.clone() 
        },
         pychopper_config, paf_reads.clone());
    let pychopper_categorised_reads = fastq::parse_categorise(
        output_fastq.clone(), 
        paf_reads.clone(), 
        true
    );

    let correct_intersection: HashSet<&String> = restrander_categorised_reads.correct
        .intersection(&pychopper_categorised_reads.correct).collect();
    let correct_difference: HashSet<&String> = restrander_categorised_reads.correct
        .difference(&pychopper_categorised_reads.correct).collect();
}

fn main() {
    // collect the input filenames from the command line args
    let input = Input::new_from_args();

    // make the configs
    // json::make_desired_configs(input.clone().config_dir, input.clone().protocol);

    // get all the configs from the given config location
    let restrander_configs = get_paths(input.clone().config_dir);
    let pychopper_configs = vec![
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::Edlib,
            protocol: input.clone().protocol
        }),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::MachineLearning,
            protocol: input.clone().protocol
        })
    ];

    // perform the grid test as configured
    let results = restrander_grid_test(vec![input.clone()], restrander_configs).into_iter()
        .chain(pychopper_grid_test(vec![input.clone()], pychopper_configs).into_iter())
        .collect_vec();

    // prettyprint it
    print_results(results);
}

// get config paths
fn get_paths(config_dir: String) -> Vec<String> {
    fs::read_dir(config_dir)
        .unwrap()
        .into_iter()
        .map(|path| -> String {path.unwrap().path().to_str().unwrap().to_string()})
        .collect_vec()
}

#[derive(Clone)]
struct Input {
    fastq: String,
    paf: String,
    config_dir: String,
    protocol: Protocol
}

impl Input {
    pub fn new_from_args() -> Input {
        let args: Vec<String> = env::args().collect();

        assert!(args.len() == 5);
        Input { fastq: args[1].clone(), paf: args[2].clone(), config_dir: args[3].clone(), protocol: Protocol::new(args[4].clone().as_str()) }
    }

    pub fn _new(fastq: String, paf: String, config_dir: String, protocol: Protocol) -> Input {
        Input { fastq, paf, config_dir, protocol }
    }
}

fn _generate_error_rates(max: f64, step: i32) -> Vec<f64> {
    let mut error_rates: Vec<f64> = vec![];
    for i in 0..step {
        error_rates.push((max / step as f64) * (i as f64));
    }
    error_rates
}

// fn restrander_generate_configs(error_rates: Vec<f64>, protocols: Vec<Protocol>) -> Vec<RestranderConfig> {
//     // make all the config files
//     let config_filenames = json::make_configs(error_rates.clone());

//     // compile it all together correctly
//     iproduct!(error_rates, protocols)
//     error_rates.iter()
//         .product(protocols)
//         .zip(config_filenames)
//         .map(|(error_rate, config_filename)| RestranderConfig{config_filename: config_filename, error_rate: *error_rate})
//         .collect()
// }

fn restrander_grid_test(inputs: Vec<Input>, configs: Vec<String>) -> Vec<config::ProgramResult> {
    // run restrander on the product of inputs and configs
    iproduct!(inputs, configs)
        .map(|(input, config)| (
            config::GenericProgramConfig{
                input: input.fastq.clone(), 
                output: constants::OUTPUT_FILENAME.to_string()
            }, 
            config, 
            paf::parse(input.paf.clone())))
        .map(|(generic_config, restrander_config, paf_reads)| 
            restrander::accuracy_timed_run_config(generic_config, restrander_config, paf_reads))
        .collect()
}

fn pychopper_grid_test(inputs: Vec<Input>, configs: Vec<SpecificProgramConfig>) -> Vec<config::ProgramResult> {
    // run pychopper on the product of inputs and configs
    iproduct!(inputs, configs)
        .map(|(input, config)| (config::GenericProgramConfig{input: input.fastq.clone(), output: constants::OUTPUT_FILENAME.to_string()}, config, paf::parse(input.paf.clone())))
        .map(|(generic_config, restrander_config, paf_reads)| 
            pychopper::accuracy_timed_run_config(generic_config, restrander_config, paf_reads))
        .collect()
}

fn print_results(results: Vec<ProgramResult>) {
    // print CSV header line
    println!("config,correct_percent,incorrect_percent,ambiguous_percent,time_secs");
    
    // print each result
    results.iter()
       .for_each(|result| println!("{},{},{},{},{}", result.config.specific, result.accuracy.correct, result.accuracy.incorrect, result.accuracy.ambiguous, result.duration));
}

// fn print_results_by_input(results: Vec<ProgramResult>) {
//     let configs = results.iter()
//         .map(|result| result.config.specific.clone())
//         .unique();
    
//     let inputs = results.iter()
//         .map(|result| result.config.generic.input.clone())
//         .unique();

//     results.iter();
        
// }

/*
fn pychopper_grid_test(inputs: Vec<Input>, configs: Vec<config::PychopperConfig>) {
    iproduct!(inputs, configs)
        .map(|(input, config)| (config::GenericProgramConfig{input: input.fastq.clone(), output: constants::OUTPUT_FILENAME.to_string()}, config, paf::parse(input.paf.clone())))
        .map(|(generic_config, pychopper_config, paf_reads)|
            pychopper::accuracy_timed_run_config(generic_config, pychopper_config, &paf_reads))
}
*/