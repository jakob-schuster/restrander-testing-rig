use std::{collections::HashSet, fs::File, io::Write};

use itertools::Itertools;

use crate::{config::{SpecificProgramConfig, GenericProgramConfig}, paf::PafReads, restrander, fastq::{self, CategorisedReads}, pychopper};

fn custom_intersection(s1: HashSet<String>, s2: HashSet<String>) -> Vec<String> {
    s1.into_iter()
        .filter(|s| s2.contains(s))
        .collect_vec()
}
fn custom_difference(s1: HashSet<String>, s2: HashSet<String>) -> Vec<String> {
    s1.into_iter()
        .filter(|s| !s2.contains(s))
        .collect_vec()
}

fn string_vec_to_file(filename: String, strings: Vec<String>) {
    File::create(filename.clone())
        .expect(format!("Failed to create file {}", filename.clone()).as_str())
        .write(strings.join("\n").as_bytes())
        .expect("Failed to write to file!");
}

struct Venn {
    intersection: Vec<String>,
    restrander_only: Vec<String>,
    pychopper_only: Vec<String>
}

impl Venn {
    fn new(reads1: HashSet<String>, reads2: HashSet<String>) -> Venn {
        Venn {
            intersection: custom_intersection(reads1.clone(), reads2.clone()),
            restrander_only: custom_difference(reads1.clone(), reads2.clone()),
            pychopper_only: custom_difference(reads2.clone(), reads1.clone())
        }
    }

    fn new_correct(restrander: CategorisedReads, pychopper: CategorisedReads) -> Venn {
        Venn::new(restrander.correct, pychopper.correct)
    }
    fn new_incorrect(restrander: CategorisedReads, pychopper: CategorisedReads) -> Venn {
        Venn::new(restrander.incorrect, pychopper.incorrect)
    }
    fn new_ambiguous(restrander: CategorisedReads, pychopper: CategorisedReads) -> Venn {
        Venn::new(restrander.ambiguous, pychopper.ambiguous)
    }

    fn to_files(self, directory: String) {
        string_vec_to_file(format!("{}/intersection.csv", directory), self.intersection);
        string_vec_to_file(format!("{}/restrander_only.csv", directory), self.restrander_only);
        string_vec_to_file(format!("{}/pychopper_only.csv", directory), self.pychopper_only);
    }
}

pub fn compare(
    input_fastq: String, 
    restrander_config: String, 
    pychopper_config: SpecificProgramConfig, 
    paf_reads: PafReads,
    temp_fastq: String,
    output_directory: String
) {
    restrander::_run(&input_fastq, &temp_fastq, &restrander_config);
    let restrander_categorised_reads = fastq::parse_categorise(
        temp_fastq.clone(),
        paf_reads.clone(),
        false
    );

    pychopper::run(
        GenericProgramConfig {
            input: input_fastq, 
            output: temp_fastq.clone() 
        },
         pychopper_config, paf_reads.clone());
    let pychopper_categorised_reads = fastq::parse_categorise(
        temp_fastq.clone(), 
        paf_reads.clone(),
        true
    );

    let correct_venn = Venn::new_correct(restrander_categorised_reads.clone(), pychopper_categorised_reads.clone());
    let incorrect_venn = Venn::new_incorrect(restrander_categorised_reads.clone(), pychopper_categorised_reads.clone());
    let ambiguous_venn = Venn::new_ambiguous(restrander_categorised_reads, pychopper_categorised_reads);

    correct_venn.to_files(format!("{}/correct", output_directory).to_string());
    incorrect_venn.to_files(format!("{}/incorrect", output_directory).to_string());
    ambiguous_venn.to_files(format!("{}/ambiguous", output_directory).to_string());
}