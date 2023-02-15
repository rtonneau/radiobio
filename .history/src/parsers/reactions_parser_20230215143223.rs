#![allow(dead_code)]

use std::fs::File;

use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reactions {
    acid_base: Vec<AcidBase>,
    k_reactions: Vec<KReaction>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f32,
}

#[derive(Debug, Deserialize)]
pub struct KReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
}

pub fn parse_reactions_file(path: &str) -> Reactions {
    let file = File::open(&path).expect("Failed Opening
        config reactions file");
    let config: Reactions = match from_reader(file){
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse reactions data file: {}", e);
            std::process::exit(1);
        }
    };
    return config;
}