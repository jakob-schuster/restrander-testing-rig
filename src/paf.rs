
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct PafRead {
    pub name: String,
    pub strand: char,
}

#[derive(Clone)]
pub struct PafReads {
    pub map: HashMap<String, char>,
    pub size: i32
}

impl PafReads {
    fn new() -> PafReads {
        PafReads { map: HashMap::new(), size: 0 }
    }

    pub fn insert(mut self, read: PafRead) {
        self.map.insert(read.name, read.strand);
        self.size += 1;
    }

    // pub fn insert(paf_reads: PafReads, read: PafRead) -> PafReads {
    //     let thing = paf_reads.map.extend(
    //         HashMap::from(paf_reads.map.));

    //     PafReads {
    //         map: 
    //         size: paf_reads.size + 1
    //     }
    // }

    pub fn get(self, name: String) -> char {
        match self.map.get(&name) {
            None => panic!("Name {} not in PAF!", name),
            Some(strand) => strand.clone()
        }
    }

    pub fn size(self) -> i32 {
        self.size
    }
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

pub fn parse (filename: String) -> PafReads {
    let paf = BufReader::new(File::open(filename).expect("open failed"));
    let mut reads = PafReads::new();

    for line in paf.lines() {
        let read = PafRead::from_paf_line(&line.unwrap().as_bytes().to_vec());
        reads.map.entry(read.name).or_insert(0 as char);
    }

    // let reads = paf.lines().into_iter()
    //     .fold(PafReads::new(), |reads, line| -> PafReads {
    //         let read = PafRead::from_paf_line(&line.unwrap().as_bytes().to_vec());
    //         let mut map = reads.map.clone();
            
    //         reads.map.entry()
    //         map.insert(read.name, read.strand);
    //         PafReads { map, size: reads.size + 1 }
    //     });
    
    reads
}