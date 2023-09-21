use seq_io::fastq::{Reader,Record};
use std::{str, collections::{HashMap, HashSet}};

#[path = "paf.rs"] mod paf;
use crate::paf::{PafRead, PafReads};

#[derive(Debug, Clone)]

pub struct AccuracyResult {
    pub correct: f64,
    pub incorrect: f64,
    pub ambiguous: f64,
}

impl AccuracyResult {
    fn new(result: &AccuracyResultExact, total: i32) -> AccuracyResult {
        AccuracyResult {
            correct: result.correct as f64 / total as f64,
            incorrect: result.incorrect as f64 / total as f64,
            ambiguous: result.ambiguous as f64 / total as f64,
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

pub fn parse (filename: String, paf_reads: PafReads, is_pychopper: bool) -> AccuracyResult {


    let size = paf_reads.size;
    let mut result_exact: AccuracyResultExact = AccuracyResultExact {
        correct: 0, 
        incorrect: 0, 
        ambiguous: 0,
    };

    let mut reader = Reader::from_path(filename).unwrap();

    // let size = paf_reads.clone().size();

    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        
        // get the name differently depending on whether this is pychopper
        let name = if is_pychopper {
            record.id().unwrap().split("|").collect::<Vec<_>>()[1]
        } else {
            record.id().unwrap().split("|").collect::<Vec<_>>()[0]
        };
        
        // skip non-matching records
        let strand = paf_reads.map.get(&name.to_string()).expect("Failed to read!").to_owned();
        let current = *record.head().last().unwrap() as char;
        
        if current == '?' {
            result_exact.ambiguous += 1;
        } else if current == strand {
            result_exact.correct += 1;
        } else {
            result_exact.incorrect += 1;
        }
    }

    result_exact.ambiguous = size as u64 - (result_exact.correct + result_exact.incorrect);

    AccuracyResult::new(&result_exact, size).to_percent()
}

#[derive(Clone)]
pub struct CategorisedReads {
    pub correct: HashSet<String>,
    pub incorrect: HashSet<String>,
    pub ambiguous: HashSet<String>
}

impl CategorisedReads {
    fn new() -> CategorisedReads {
        CategorisedReads {
            correct: HashSet::new(),
            incorrect: HashSet::new(),
            ambiguous: HashSet::new()
        }
    }
}

pub fn parse_categorise (filename: String, paf_reads: PafReads, is_pychopper: bool) -> CategorisedReads {
    let mut reader = Reader::from_path(filename).unwrap();

    let mut fastq_reads: HashMap<String, char> = HashMap::new();
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let name = if is_pychopper {
            record.id().unwrap().split("|").collect::<Vec<_>>()[1]
        } else {
            // restrander it goes to the 0 spot
            record.id().unwrap().split("|").collect::<Vec<_>>()[0]
        };
        
        // skip non-matching records
        let current = *record.head().last().unwrap();

        fastq_reads.insert(name.to_string(), current.clone() as char);
    }

    let mut reads = CategorisedReads::new();
    for paf_key in paf_reads.map.keys() {
        // skip ambiguous entries
        if !fastq_reads.contains_key(paf_key) {
            reads.ambiguous.insert(paf_key.clone());
            continue;
        }

        // categorise!
        if fastq_reads.contains_key(paf_key) {
            if *paf_reads.map.get(&paf_key.clone()).expect("Paf map didn't have key") == *fastq_reads.get(&paf_key.clone()).expect("Fastq map didn't have key") {
                reads.correct.insert(paf_key.clone());
            } else {
                reads.incorrect.insert(paf_key.clone());
            }
        }
    }

    reads
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