use crate::fastq::AccuracyResult;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ProgramResult {
    pub config: ProgramConfig,
    pub duration: u64,
    pub accuracy: AccuracyResult,
}

#[derive(Debug, Clone)]
pub struct GenericProgramConfig {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct ProgramConfig {
    pub specific: SpecificProgramConfig,
    pub generic: GenericProgramConfig,
}

impl fmt::Display for ProgramConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.specific.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum SpecificProgramConfig {
    Restrander(RestranderConfig),
    Pychopper(PychopperConfig)
}

impl fmt::Display for SpecificProgramConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecificProgramConfig::Restrander(config) => {
                write!(f, "Restrander({})", config.error_rate)
            }
            SpecificProgramConfig::Pychopper(config) => {
                write!(f, "Pychopper({:?})", config.backend)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PychopperConfig {
    pub backend: PychopperBackend,
}

#[derive(Debug, Clone)]
pub struct RestranderConfig {
    pub config_filename: String,
    pub error_rate: f64,
}

#[derive(Debug, Clone)]
pub enum PychopperBackend {
    Edlib,
    MachineLearning
}