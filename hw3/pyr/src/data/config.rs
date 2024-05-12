use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = {
        match std::fs::read_to_string("config.toml"){
            Ok(content)=>toml::from_str(&content).unwrap(),
            Err(_)=>Config::default()
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub exploration_rate: u32,
    pub update_frequency: usize,
    pub batch_size: usize,
    pub replay_size: usize,
    pub learning_rate: f64,
    pub gamma: f64,
    pub train: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exploration_rate: 1024,
            update_frequency: 150,
            batch_size: 32,
            replay_size: 250,
            learning_rate: 0.04,
            gamma: 0.99,
            train: false,
        }
    }
}
