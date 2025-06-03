use std::{collections::HashSet, fs::File, io::Write};

use itertools::Itertools;

use crate::{
    config::{GenericProgramConfig, SpecificProgramConfig},
    fastq::{self, CategorisedReads},
    paf::PafReads,
    pychopper, restrander,
};

fn custom_intersection(s1: &HashSet<String>, s2: &HashSet<String>) -> Vec<String> {
    s1.iter().filter(|s| s2.contains(*s)).cloned().collect_vec()
}
fn custom_difference(s1: &HashSet<String>, s2: &HashSet<String>) -> Vec<String> {
    s1.iter()
        .filter(|s| !s2.contains(*s))
        .cloned()
        .collect_vec()
}

fn string_vec_to_file(filename: &str, strings: &[String]) {
    File::create(filename)
        .expect(&format!("Failed to create file {}", filename))
        .write_all(strings.join("\n").as_bytes())
        .expect("Failed to write to file!");
}

struct Venn {
    intersection: Vec<String>,
    restrander_only: Vec<String>,
    pychopper_only: Vec<String>,
}

impl Venn {
    fn new(reads1: &HashSet<String>, reads2: &HashSet<String>) -> Venn {
        Venn {
            intersection: custom_intersection(reads1, reads2),
            restrander_only: custom_difference(reads1, reads2),
            pychopper_only: custom_difference(reads2, reads1),
        }
    }

    fn new_correct(restrander: &CategorisedReads, pychopper: &CategorisedReads) -> Venn {
        Venn::new(&restrander.correct, &pychopper.correct)
    }
    fn new_incorrect(restrander: &CategorisedReads, pychopper: &CategorisedReads) -> Venn {
        Venn::new(&restrander.incorrect, &pychopper.incorrect)
    }
    fn new_ambiguous(restrander: &CategorisedReads, pychopper: &CategorisedReads) -> Venn {
        Venn::new(&restrander.ambiguous, &pychopper.ambiguous)
    }

    fn to_files(&self, directory: &str) {
        string_vec_to_file(
            &format!("{}/intersection.csv", directory),
            &self.intersection,
        );
        string_vec_to_file(
            &format!("{}/restrander_only.csv", directory),
            &self.restrander_only,
        );
        string_vec_to_file(
            &format!("{}/pychopper_only.csv", directory),
            &self.pychopper_only,
        );
    }
}

pub fn compare(
    input_fastq: &str,
    restrander_config: &str,
    pychopper_config: &SpecificProgramConfig,
    paf_reads: &PafReads,
    temp_fastq: &str,
    output_directory: &str,
) {
    restrander::_run(input_fastq, temp_fastq, restrander_config);
    let restrander_categorised_reads = fastq::parse_categorise(temp_fastq, paf_reads, false);

    pychopper::run(
        &GenericProgramConfig {
            input: input_fastq.to_string(),
            output: temp_fastq.to_string(),
        },
        pychopper_config,
        paf_reads,
    );
    let pychopper_categorised_reads = fastq::parse_categorise(temp_fastq, paf_reads, true);

    let correct_venn =
        Venn::new_correct(&restrander_categorised_reads, &pychopper_categorised_reads);
    let incorrect_venn =
        Venn::new_incorrect(&restrander_categorised_reads, &pychopper_categorised_reads);
    let ambiguous_venn =
        Venn::new_ambiguous(&restrander_categorised_reads, &pychopper_categorised_reads);

    correct_venn.to_files(&format!("{}/correct", output_directory));
    incorrect_venn.to_files(&format!("{}/incorrect", output_directory));
    ambiguous_venn.to_files(&format!("{}/ambiguous", output_directory));
}
