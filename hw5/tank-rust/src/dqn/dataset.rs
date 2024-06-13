use std::{
    env::{self},
    fs::File,
    io::BufReader,
};

use crate::ffi::prelude::*;
use crate::ARTIFACT_DIR;
use burn::{
    data::{dataloader::batcher::Batcher, dataset::Dataset},
    prelude::*,
};

use super::feature::FEATRUE_SPACE;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TankItem {
    pub previous_state: [f32; FEATRUE_SPACE],
    pub new_state: [f32; FEATRUE_SPACE],
    pub action: Action,
    pub reward: f32,
}

pub struct TankDataset {
    pub dataset: Vec<TankItem>,
}

impl Dataset<TankItem> for TankDataset {
    fn get(&self, index: usize) -> Option<TankItem> {
        self.dataset.get(index).cloned()
    }

    fn len(&self) -> usize {
        self.dataset.len()
    }
}

impl TankDataset {
    pub fn new() -> Self {
        Self {
            dataset: Vec::new(),
        }
    }
    pub fn load() -> Self {
        let dataset_path = env::var("MODEL_PATH").unwrap_or_else(|_| ARTIFACT_DIR.to_string());
        let mut dataset = Vec::new();
        println!("Loading dataset from: {}", dataset_path);
        if let Ok(reader) = File::open(format!("{dataset_path}/dataset")) {
            let mut reader = BufReader::new(reader);
            while let Ok(item) = bincode::deserialize_from::<_, TankItem>(&mut reader) {
                dataset.push(item);
            }
        }
        TankDataset { dataset }
    }
    pub fn add(&mut self, item: TankItem) {
        self.dataset.push(item);
    }
    pub fn split(self, ratio: f32) -> (Self, Self) {
        let split = (self.dataset.len() as f32 * ratio).round() as usize;
        let (a, b) = self.dataset.split_at(split);
        (
            TankDataset {
                dataset: a.to_vec(),
            },
            TankDataset {
                dataset: b.to_vec(),
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct TankBatcher<B: Backend> {
    device: B::Device,
}

#[derive(Clone, Debug)]
pub struct TankBatch<B: Backend> {
    pub new_state: Tensor<B, 2>,
    pub old_state: Tensor<B, 2>,
    pub action: Tensor<B, 1, Int>,
    pub reward: Tensor<B, 1>,
}

impl<B: Backend> TankBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<TankItem, TankBatch<B>> for TankBatcher<B> {
    fn batch(&self, items: Vec<TankItem>) -> TankBatch<B> {
        let mut new_state: Vec<Tensor<B, 2>> = Vec::new();
        let mut old_state: Vec<Tensor<B, 2>> = Vec::new();

        for item in items.iter() {
            let new_state_tensor = Tensor::<B, 1>::from_floats(item.previous_state, &self.device);
            let old_state_tensor = Tensor::<B, 1>::from_floats(item.new_state, &self.device);

            new_state.push(new_state_tensor.unsqueeze());
            old_state.push(old_state_tensor.unsqueeze());
        }

        let new_state = Tensor::cat(new_state, 0);
        let old_state = Tensor::cat(old_state, 0);

        let reward = items
            .iter()
            .map(|item| Tensor::<B, 1, Float>::from_floats([item.reward], &self.device))
            .collect();
        let reward = Tensor::cat(reward, 0);

        let response = items
            .iter()
            .map(|item| Tensor::<B, 1, Int>::from_ints([item.action as i32], &self.device))
            .collect();
        let response = Tensor::cat(response, 0);

        TankBatch {
            new_state,
            old_state,
            reward,
            action: response,
        }
    }
}
