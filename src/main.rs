use core::panic;
use std::{env, fs};

use config::{
    GenericProgramConfig, ProgramResult, Protocol, PychopperConfig, SpecificProgramConfig,
};
use itertools::{iproduct, Itertools};
use restrander::accuracy_timed_run_config;

mod comparison;
mod config;
mod constants;
mod fastq;
mod json;
mod paf;
mod pychopper;
mod restrander;

enum ProgramInput {
    GridTest {
        fastq: String,
        paf: String,
        config_dir: String,
        temp_fastq: String,
        protocol: Protocol,
    },
    CompareReads {
        fastq: String,
        paf: String,
        restrander_config: String,
        temp_fastq: String,
        output_directory: String,
        protocol: Protocol,
    },
    Standard {
        fastq: String,
        paf: String,
        restrander_config: String,
        temp_fastq: String,
        protocol: Protocol,
    },
    Quick {
        fastq: String,
        paf: String,
    },
}

impl ProgramInput {
    fn new_from_args() -> ProgramInput {
        let args: Vec<String> = env::args().collect();

        if args.is_empty() {
            panic!("No argument given!")
        }

        match args[1].trim() {
            "grid" => ProgramInput::GridTest {
                fastq: args[2].clone(),
                paf: args[3].clone(),
                config_dir: args[4].clone(),
                temp_fastq: args[5].clone(),
                protocol: Protocol::new(args[6].clone().as_str()),
            },
            "compare" => ProgramInput::CompareReads {
                fastq: args[2].clone(),
                paf: args[3].clone(),
                restrander_config: args[4].clone(),
                temp_fastq: args[5].clone(),
                output_directory: args[6].clone(),
                protocol: Protocol::new(args[7].clone().as_str()),
            },
            "standard" => ProgramInput::Standard {
                fastq: args[2].clone(),
                paf: args[3].clone(),
                restrander_config: args[4].clone(),
                temp_fastq: args[5].clone(),
                protocol: Protocol::new(args[6].clone().as_str()),
            },
            "quick" => ProgramInput::Quick {
                fastq: args[2].clone(),
                paf: args[3].clone(),
            },
            _ => panic!("Invalid first argument: {}", args[1]),
        }
    }
}

fn main() {
    // load the generic input
    let input = ProgramInput::new_from_args();

    // send the program down the appropriate branch
    match input {
        ProgramInput::GridTest {
            fastq,
            paf,
            config_dir,
            temp_fastq,
            protocol,
        } => grid_test(&GridTestInput {
            fastq,
            paf,
            config_dir,
            temp_fastq,
            protocol,
        }),
        ProgramInput::CompareReads {
            fastq,
            paf,
            restrander_config,
            temp_fastq,
            output_directory,
            protocol,
        } => compare(
            &fastq,
            &paf,
            &restrander_config,
            &temp_fastq,
            &output_directory,
            &protocol,
        ),
        ProgramInput::Standard {
            fastq,
            paf,
            restrander_config,
            temp_fastq,
            protocol,
        } => standard(&fastq, &paf, &restrander_config, &temp_fastq, &protocol),
        ProgramInput::Quick { fastq, paf } => quick(&fastq, &paf),
    }
}

fn quick(fastq: &str, paf: &str) {
    let paf_reads = paf::parse(paf);

    let result = fastq::parse(fastq, &paf_reads, false);

    println!("{}", result);
}

fn compare(
    fastq: &str,
    paf: &str,
    restrander_config: &str,
    temp_fastq: &str,
    output_directory: &str,
    protocol: &Protocol,
) {
    let pychopper_config = SpecificProgramConfig::Pychopper(PychopperConfig {
        backend: config::PychopperBackend::MachineLearning,
        protocol: protocol.clone(),
    });

    let paf_reads = paf::parse(paf);

    comparison::compare(
        fastq,
        restrander_config,
        &pychopper_config,
        &paf_reads,
        temp_fastq,
        output_directory,
    );
}

fn standard(
    fastq: &str,
    paf: &str,
    restrander_config: &str,
    temp_fastq: &str,
    protocol: &Protocol,
) {
    let paf_reads = paf::parse(paf);
    let generic_config: GenericProgramConfig = GenericProgramConfig {
        input: fastq.to_string(),
        output: temp_fastq.to_string(),
    };

    let restrander_result =
        accuracy_timed_run_config(&generic_config, restrander_config, &paf_reads);

    let pychopper_edlib_result = pychopper::accuracy_timed_run_config(
        &generic_config,
        &SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::Edlib,
            protocol: protocol.clone(),
        }),
        &paf_reads,
    );
    let pychopper_ml_result = pychopper::accuracy_timed_run_config(
        &generic_config,
        &SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::MachineLearning,
            protocol: protocol.clone(),
        }),
        &paf_reads,
    );

    print_results(&[
        restrander_result,
        pychopper_edlib_result,
        pychopper_ml_result,
    ]);
}

fn grid_test(input: &GridTestInput) {
    // make the configs
    json::pcb111_protocol_testing(&input.clone().config_dir, &input.clone().protocol);

    // get all the configs from the given config location
    let restrander_configs = get_paths(&input.clone().config_dir);
    let pychopper_configs = vec![
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::Edlib,
            protocol: input.clone().protocol,
        }),
        SpecificProgramConfig::Pychopper(PychopperConfig {
            backend: config::PychopperBackend::MachineLearning,
            protocol: input.clone().protocol,
        }),
    ];

    // perform the grid test as configured
    let results = restrander_grid_test(&[input.clone()], &restrander_configs)
        .into_iter()
        .chain(pychopper_grid_test(&[input.clone()], &pychopper_configs))
        .collect_vec();

    // prettyprint it
    print_results(&results);
}

// get config paths
fn get_paths(config_dir: &str) -> Vec<String> {
    fs::read_dir(config_dir)
        .unwrap()
        .map(|path| -> String { path.unwrap().path().to_str().unwrap().to_string() })
        .collect_vec()
}

#[derive(Clone)]
struct GridTestInput {
    fastq: String,
    paf: String,
    config_dir: String,
    temp_fastq: String,
    protocol: Protocol,
}

impl GridTestInput {
    pub fn new_from_args() -> GridTestInput {
        let args: Vec<String> = env::args().collect();

        assert!(args.len() == 6);
        GridTestInput {
            fastq: args[1].clone(),
            paf: args[2].clone(),
            config_dir: args[3].clone(),
            temp_fastq: args[4].clone(),
            protocol: Protocol::new(args[5].clone().as_str()),
        }
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

fn restrander_grid_test(
    inputs: &[GridTestInput],
    configs: &[String],
) -> Vec<config::ProgramResult> {
    // run restrander on the product of inputs and configs
    iproduct!(inputs, configs)
        .map(|(input, config)| {
            (
                config::GenericProgramConfig {
                    input: input.fastq.clone(),
                    output: input.temp_fastq.to_string(),
                },
                config,
                paf::parse(&input.paf),
            )
        })
        .map(|(generic_config, restrander_config, paf_reads)| {
            restrander::accuracy_timed_run_config(&generic_config, restrander_config, &paf_reads)
        })
        .collect()
}

fn pychopper_grid_test(
    inputs: &[GridTestInput],
    configs: &[SpecificProgramConfig],
) -> Vec<config::ProgramResult> {
    // run pychopper on the product of inputs and configs
    iproduct!(inputs, configs)
        .map(|(input, config)| {
            (
                config::GenericProgramConfig {
                    input: input.fastq.clone(),
                    output: input.temp_fastq.clone(),
                },
                config,
                paf::parse(&input.paf),
            )
        })
        .map(|(generic_config, restrander_config, paf_reads)| {
            pychopper::accuracy_timed_run_config(&generic_config, restrander_config, &paf_reads)
        })
        .collect()
}

fn print_results(results: &[ProgramResult]) {
    // print CSV header line
    println!("config,correct_percent,incorrect_percent,ambiguous_percent,time_secs");

    // print each result
    results.iter().for_each(|result| {
        println!(
            "{},{},{},{},{}",
            result.config.specific,
            result.accuracy.correct,
            result.accuracy.incorrect,
            result.accuracy.ambiguous,
            result.duration
        )
    });
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
