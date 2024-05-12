use std::collections::VecDeque;
use std::path::Path;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use candle_core::{DType, Device, Module, Tensor};
use candle_nn::{linear, seq, Activation, AdamW, Optimizer, Sequential, VarBuilder, VarMap};

use crate::CONFIG;

use super::state::OBSERVATION_SPACE;
use super::{action::AIAction, huber::huber_loss, state::AIState};

// const DEVICE: Device = Device::Cpu;

const ACTION_SPACE: usize = 5;

pub struct AIAgent {
    var_map: VarMap,
    model: Sequential,
    optimizer: AdamW,
    memory: VecDeque<(Tensor, u32, Tensor, f64)>,
    old_state: Option<AIState>,
    step: usize,
    device: Device,
    accumulate_rewards: f64,
}

impl AIAgent {
    pub async fn new() -> Self {
        #[cfg(not(feature = "cuda"))]
        let device=Device::Cpu;
        #[cfg(feature = "cuda")]
        let device=Device::new_cuda(0).unwrap();

        let mut var_map = VarMap::new();
        if Path::new("model.bin").exists() {
            var_map.load("model.bin").unwrap();
        }
        let vb = VarBuilder::from_varmap(&var_map, DType::F32, &device);
        let model = seq()
            .add(linear(OBSERVATION_SPACE, 60, vb.pp("linear_in")).unwrap())
            .add(Activation::LeakyRelu(0.01))
            .add(linear(60, 48, vb.pp("linear_mid_1")).unwrap())
            .add(Activation::LeakyRelu(0.01))
            .add(linear(48, 48, vb.pp("linear_mid_2")).unwrap())
            .add(Activation::LeakyRelu(0.01))
            .add(linear(48, ACTION_SPACE, vb.pp("linear_out")).unwrap())
            .add(Activation::LeakyRelu(0.01));

        let optimizer = AdamW::new_lr(var_map.all_vars(), CONFIG.learning_rate).unwrap();

        Self {
            var_map,
            model,
            optimizer,
            memory: VecDeque::new(),
            old_state: None,
            step: 0,
            device,
            accumulate_rewards: 0.0,
        }
    }
    fn get_reward(&self, new_state: &AIState) -> f64 {
        let old_state = self.old_state.as_ref().unwrap();
        // let new_positive_distance = new_state
        //     .get_postivie_food()
        //     .map(|food| food.x + food.y)
        //     .unwrap_or(0.0);
        // let old_positive_distance = old_state
        //     .get_postivie_food()
        //     .map(|food| food.x + food.y)
        //     .unwrap_or(0.0);
        // let new_negative_distance = new_state
        //     .get_negative_food()
        //     .map(|food| food.x + food.y)
        //     .unwrap_or(0.0);
        // let old_negative_distance = old_state
        //     .get_negative_food()
        //     .map(|food| food.x + food.y)
        //     .unwrap_or(0.0);

        return
        //  (old_positive_distance - new_positive_distance) as f64
            // + (new_negative_distance - old_negative_distance) as f64
            100.0*(new_state.player.score - old_state.player.score) as f64;
    }
    pub fn tick(&mut self, state: AIState) -> AIAction {
        self.step += 1;
        if self.old_state.is_none() {
            self.old_state = Some(state);
            return AIAction::None;
        }
        let old_state = self.old_state.as_ref().unwrap();

        let action: u32 = match thread_rng().gen_ratio(CONFIG.exploration_rate, 4096) {
            true if CONFIG.train => thread_rng().gen_range(0..(ACTION_SPACE as u32)),
            _ => self
                .model
                .forward(&old_state.into_tensor(&self.device))
                .unwrap()
                .squeeze(0)
                .unwrap()
                .argmax(0)
                .unwrap()
                .to_scalar()
                .unwrap(),
        };

        if CONFIG.train {
            let reward = self.get_reward(&state);
            self.accumulate_rewards += reward;

            self.memory.push_front((
                self.old_state
                    .as_ref()
                    .unwrap()
                    .into_tensor(&self.device)
                    .squeeze(0)
                    .unwrap(),
                action,
                state.into_tensor(&self.device).squeeze(0).unwrap(),
                reward,
            ));
            self.memory.truncate(CONFIG.replay_size);
            if self.step % CONFIG.update_frequency == 0 && self.memory.len() > CONFIG.batch_size {
                self.train();
            }
        }

        self.old_state = Some(state);

        match action {
            0 => AIAction::None,
            1 => AIAction::Up,
            2 => AIAction::Left,
            3 => AIAction::Right,
            _ => AIAction::Down,
        }
    }
    fn train(&mut self) {
        // Sample randomly from the memory.
        let batch = thread_rng()
            .sample_iter(Uniform::from(0..self.memory.len()))
            .take(CONFIG.batch_size)
            .map(|i| self.memory.get(i).unwrap().clone())
            .collect::<Vec<_>>();

        // Group all the samples together into tensors with the appropriate shape.
        let states: Vec<_> = batch.iter().map(|e| e.0.clone()).collect();
        let states = Tensor::stack(&states, 0).unwrap();

        let actions = batch.iter().map(|e| e.1);
        let actions = Tensor::from_iter(actions, &self.device)
            .unwrap()
            .unsqueeze(1)
            .unwrap();

        let next_states: Vec<_> = batch.iter().map(|e| e.2.clone()).collect();
        let next_states = Tensor::stack(&next_states, 0).unwrap();

        let rewards = batch.iter().map(|e| e.3 as f32);
        let rewards = Tensor::from_iter(rewards, &self.device)
            .unwrap()
            .unsqueeze(1)
            .unwrap();

        let non_final_mask = batch.iter().map(|_| true as u8 as f32);
        let non_final_mask = Tensor::from_iter(non_final_mask, &self.device)
            .unwrap()
            .unsqueeze(1)
            .unwrap();

        // Get the estimated rewards for the actions that where taken at each step.
        let estimated_rewards = self.model.forward(&states).unwrap();
        let x = estimated_rewards.gather(&actions, 1).unwrap();

        // Get the maximum expected rewards for the next state, apply them a discount rate
        // GAMMA and add them to the rewards that were actually gathered on the current state.
        // If the next state is a terminal state, just omit maximum estimated
        // rewards for that state.
        let expected_rewards = self.model.forward(&next_states).unwrap().detach();
        let y = expected_rewards.max_keepdim(1).unwrap();
        let y = (y * CONFIG.gamma * non_final_mask + rewards).unwrap();

        // Compare the estimated rewards with the maximum expected rewards and
        // perform the backward step.
        let loss = huber_loss(1.0_f32)(&x, &y);
        self.optimizer
            .backward_step(&Tensor::new(&[loss], &self.device).unwrap())
            .unwrap();
    }
    pub fn check_point(&mut self) {
        self.memory.clear();
        if CONFIG.train {
            self.var_map.save("model.bin").unwrap();
            log::info!("Rewards {}", self.accumulate_rewards as i64);
            self.accumulate_rewards = 0.0;
            log::info!("model.bin saved!");
        }
    }
}

// impl Drop for AIAgent {
//     fn drop(&mut self) {
//         self.var_map.save("model.bin").unwrap();
//         log::info!("model.bin saved!");
//         log::info!("Rewards {}", self.accumulate_rewards as i64);
//     }
// }
