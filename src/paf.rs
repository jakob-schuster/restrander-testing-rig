
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct PafRead {
    pub name: String,
    pub strand: char,
}

impl PafRead {
    pub fn from_paf_line (line: &Vec<u8>) -> PafRead {
        let mut name: String = String::new();
        let mut i: usize = 0;
        while i < line.len() {
            let c = line[i] as char;
            if c != '\t' {
                name.push(c);
            } else {
                break;
            }

            i += 1;
        }
        
        let mut strand: char = '?';
        while i < line.len() {
            let c = line[i] as char;
            if c == '+' || c == '-' {
                strand = c;
                break;
            }

            i += 1;
        }

        let read = PafRead {
            name,
            strand,
        };

        return read;
    }

    fn _print (read: &PafRead) {
        println!("{}, {}", read.name, read.strand);
    }
}

pub fn parse (filename: String) -> Vec<PafRead> {
    let paf = BufReader::new(File::open(filename).expect("open failed"));

    let mut reads: Vec<PafRead> = Vec::new();
    for line in paf.lines() {
        reads.push(PafRead::from_paf_line(&line.unwrap().as_bytes().to_vec()));
    }

    return reads;
}