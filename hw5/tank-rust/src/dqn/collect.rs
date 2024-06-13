use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

use super::dataset::TankItem;
use super::feature::ACTION_SPACE;
use crate::{ffi::prelude::*, Backend};
use burn::{backend::wgpu::WgpuDevice, module::Module, record::NoStdTrainingRecorder};
use rand::{thread_rng, Rng};

use super::model::{DQNModel, DQNModelConfig};
pub struct App<'a> {
    model: DQNModel<Backend>,
    device: WgpuDevice,
    last_state_action: Option<(Info<'a>, Action)>,
    #[cfg(feature = "train")]
    outlet: BufWriter<File>,
    #[cfg(feature = "train")]
    explore_rate: f32,
}

impl<'a> App<'a> {
    pub fn new(model_path: &str) -> Self {
        let device = burn::backend::wgpu::WgpuDevice::default();

        let model = DQNModelConfig::new().init(&device);

        let model = model
            .load_file(
                format!("{model_path}/model"),
                &NoStdTrainingRecorder::new(),
                &device,
            )
            .unwrap();
        #[cfg(feature = "train")]
        let explore_rate = std::env::var("EPSILON")
            .map(|x| {
                let n: usize = x.parse().ok()?;
                Some(1.0 / (n as f32 + 2.0).log2() - 0.03)
            })
            .into_iter()
            .flatten()
            .next()
            .unwrap_or(0.4);

        Self {
            model,
            device,
            last_state_action: None,
            #[cfg(feature = "train")]
            outlet: BufWriter::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(format!("{model_path}/dataset"))
                    .unwrap(),
            ),
            #[cfg(feature = "train")]
            explore_rate,
        }
    }
    #[cfg(feature = "train")]
    pub fn flush(&mut self) {
        self.outlet.flush().unwrap();
    }
    #[cfg(feature = "train")]
    pub fn collect_data(&mut self, state: &Info<'static>) -> Action {
        if let Some((previous_state, action)) = self.last_state_action.take() {
            let reward = previous_state.get_reward(state, action);
            let item = TankItem {
                previous_state: previous_state.into_feature(),
                new_state: state.into_feature(),
                action,
                reward,
            };
            bincode::serialize_into(&mut self.outlet, &item).unwrap();
        }

        let action = match thread_rng().gen_ratio((4096.0 * self.explore_rate) as u32, 4096) {
            true => match thread_rng().gen_range(0..(ACTION_SPACE + 2) as i32) {
                0 => Action::TurnRight,
                1 => Action::TurnLeft,
                2 => Action::AimRight,
                3 => Action::AimLeft,
                4 => Action::Shoot,
                _ => Action::Forward,
            },
            false => self.predict_action(state),
        };

        self.last_state_action = Some((state.clone(), action));

        action
    }
    pub fn predict_action(&self, state: &Info) -> Action {
        let input = state.into_feature_tensor(&self.device).unsqueeze(); // Convert input tensor to shape [1, input_size]
        let ans = self.model.forward(input);
        match ans.argmax(1).into_scalar() {
            0 => Action::TurnRight,
            1 => Action::TurnLeft,
            2 => Action::AimRight,
            3 => Action::AimLeft,
            4 => Action::Shoot,
            5 => Action::Forward,
            _ => unreachable!("Invalid action"),
        }
    }
}
