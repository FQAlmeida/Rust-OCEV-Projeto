use std::{fmt::Debug, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PopType {
    Binary,
    Real,
    Integer,
    Permuted,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SelectionMethod {
    Roulette,
    Tournament,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CrossoverMethod {
    OnePoint,
    TwoPoints,
    Uniform,
    Cycle,
    PartiallyMapped,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BoundConfig {
    pub upper: f64,
    pub lower: f64,
}
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PopConfig {
    pub dim: usize,
    pub pop_size: usize,
    pub pop_type: PopType,
    pub bounds: Option<BoundConfig>,
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct Config {
    pub pop_config: PopConfig,
    pub qtd_gen: usize,
    pub qtd_runs: usize,
    pub generations_to_genocide: usize,
    pub elitism: bool,
    pub selection_method: SelectionMethod,
    pub crossover_method: CrossoverMethod,
    pub crossover_chance: f64,
    pub mutation_chance: f64,
    pub constraint_penalty: f64,
    pub kp: f64,
    pub generation_gap: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            pop_config: PopConfig {
                dim: 100,
                pop_size: 10,
                pop_type: PopType::Binary,
                bounds: None,
            },
            qtd_gen: 100,
            qtd_runs: 3,
            generations_to_genocide: 250,
            elitism: true,
            selection_method: SelectionMethod::Roulette,
            crossover_method: CrossoverMethod::TwoPoints,
            crossover_chance: 0.9,
            mutation_chance: 0.03,
            constraint_penalty: -1.0,
            kp: 0.9,
            generation_gap: 0.6,
        }
    }
}

impl Config {
    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    ///
    /// # Panics
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let config_str = fs::read_to_string(path)?;
        let config_json: Value = serde_json::from_str(&config_str).expect("Config file wrong format");
        let config_data: Config = serde_json::from_value(config_json.get("config").expect("Config file doesnt have config member").to_owned())?;
        Ok(config_data)
    }
}
