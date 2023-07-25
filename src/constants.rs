use std::path::MAIN_SEPARATOR;
use const_format::formatcp;

struct LightPath {
    path: String,
    children: Vec<LightPath>,
}

// static ROOT: LightPath = LightPath{path: "..".to_string(), children: vec![]};

macro_rules! path_join {
    ($x:expr, $y:expr) => {
        formatcp!("{}{}{}", $x, MAIN_SEPARATOR, $y)
    };
}

const ROOT_PATH: &str = "..";

pub const OUTPUT_FILENAME: &str = "restrander_testing_rig_out.fq";

pub const RESTRANDER_PATH: &str = path_join!(
    path_join!(ROOT_PATH, "restrander"), "restrander");
pub const PYCHOPPER_PATH: &str = path_join!(
    path_join!(ROOT_PATH, "pychopper"), "pychopper");

pub const DATA_PATH: &str = path_join!(
    path_join!(
        path_join!(ROOT_PATH, "data"), 
            "guppy5_sup_rebasecall_Mar22"), 
        "barcode01_pass");
pub const INPUT_NAME: &str = "aligned_reads.fq.gz";
pub const OUTPUT_PATH: &str = path_join!(DATA_PATH, "restrander");
pub const CONFIG_PATH: &str = path_join!(ROOT_PATH, "configs");