use seq_io::fastq::{Reader,Record};
use std::str;

#[path = "paf.rs"] mod paf;
use crate::paf::PafRead;

#[derive(Debug, Clone)]

pub struct AccuracyResult {
    pub correct: f64,
    pub incorrect: f64,
    pub ambiguous: f64,
}

impl AccuracyResult {
    fn new(result: &AccuracyResultExact) -> AccuracyResult {
        AccuracyResult {
            correct: result.correct as f64 / result.total() as f64,
            incorrect: result.incorrect as f64 / result.total() as f64,
            ambiguous: result.ambiguous as f64 / result.total() as f64,
        }
    }

    fn to_percent(&self) -> AccuracyResult {
        AccuracyResult { 
            correct: self.correct * 100 as f64, 
            incorrect: self.incorrect * 100 as f64, 
            ambiguous: self.ambiguous * 100 as f64 
        }
    }
}

pub struct AccuracyResultExact {
    pub correct: u64,
    pub incorrect: u64,
    pub ambiguous: u64,
}

impl AccuracyResultExact {
    fn total(&self) -> u64 {
        self.correct + self.incorrect + self.ambiguous
    }
}

pub fn parse_restrander (filename: String, paf_reads: &Vec<PafRead>) -> AccuracyResult {
    let mut result_exact: AccuracyResultExact = AccuracyResultExact {
        correct: 0, 
        incorrect: 0, 
        ambiguous: 0,
    };

    let mut reader = Reader::from_path(filename).unwrap();

    let mut i: usize = 0;

    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let name = record.id().unwrap().split("|").collect::<Vec<_>>()[0];
        
        // skip non-matching records
        while name != paf_reads[i].name {
            i += 1;
        }

        let current = *record.head().last().unwrap();
        if current == 63 {
            result_exact.ambiguous += 1;
        } else if current == paf_reads[i].strand as u8 {
            result_exact.correct += 1;
        } else {
            result_exact.incorrect += 1;
        }
        i += 1;
    }

    AccuracyResult::new(&result_exact).to_percent()
}

pub fn parse_pychopper (filename: String, paf_reads: &Vec<PafRead>) {

    let mut correct = 0;
    let mut incorrect = 0;
    let mut ambiguous = 0;

    let mut reader = Reader::from_path(filename).unwrap();

    let mut i: usize = 0;
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        
        let name = record.id().unwrap().split("|").collect::<Vec<_>>()[1];
        println!("{} vs {}", name, paf_reads[i].name);
        while name != paf_reads[i].name {
            println!("{} vs {} not the same so incrementing", name, paf_reads[i].name);
            i += 1;
        }
        println!("{} vs {} are the same", name, paf_reads[i].name);
        
        // print!("{} vs {}\n", record.head().last().unwrap(), paf_reads[i].strand as u8);
        let current = *record.head().last().unwrap();
        if current == 63 {
            ambiguous += 1;
        } else if current == paf_reads[i].strand as u8 {
            correct += 1;
        } else {
            incorrect += 1;
        }
        i += 1;
    }

    print!("Correct: {}\nIncorrect: {}\nAmbiguous: {}\n", correct, incorrect, ambiguous);

}

pub fn parse_nanoprep (filename: String, paf_reads: &Vec<PafRead>) {

    let mut correct = 0;
    let mut incorrect = 0;
    let mut ambiguous = 0;

    let mut reader = Reader::from_path(filename).unwrap();

    let mut i: usize = 0;
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");

        let name = record.id().unwrap();
        println!("{} vs {}", name, paf_reads[i].name);
        while name != paf_reads[i].name {
            println!("{} vs {} not the same so incrementing", name, paf_reads[i].name);
            i += 1;
        }

        let record_string = match str::from_utf8(record.head()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // extract the strand from the tag line
        let strand = record_string
            .split(" ").nth(7).unwrap()
            .as_bytes()[7];

        if strand != '-' as u8 && paf_reads[i].strand == '-' {
            incorrect += 1;
            println!("incorrect!");
        } else {
            correct += 1;
            println!("correct!");
        }
        ambiguous += 1;

        // println!("{}", strand_tag);
    }

    print!("Correct: {}\nIncorrect: {}\nAmbiguous: {}\n", correct, incorrect, ambiguous);
}