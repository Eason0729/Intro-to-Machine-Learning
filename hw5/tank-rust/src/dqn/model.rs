use super::{
    dataset::TankBatch,
    feature::{ACTION_SPACE, FEATRUE_SPACE},
};
use burn::{
    nn::{
        loss::{HuberLoss, HuberLossConfig, Reduction::Mean},
        Linear, LinearConfig, Relu,
    },
    prelude::*,
    tensor::backend::AutodiffBackend,
    train::{
        metric::{Adaptor, LossInput},
        TrainOutput, TrainStep, ValidStep,
    },
};

pub struct DQNOutput<B: Backend> {
    estimated_reward: Tensor<B, 2>,
    expected_reward: Tensor<B, 2>,
    loss: Tensor<B, 1>,
}

impl<B: Backend> Adaptor<LossInput<B>> for DQNOutput<B> {
    fn adapt(&self) -> LossInput<B> {
        LossInput::new(self.loss.clone())
    }
}

#[derive(Module, Debug)]
pub struct DQNModel<B: Backend> {
    input_layer: Linear<B>,
    hidden_layer_1: Linear<B>,
    hidden_layer_2: Linear<B>,
    output_layer: Linear<B>,
    activation: Relu,
    loss_function: HuberLoss<B>,
    gamma: f32,
}

#[derive(Config)]
pub struct DQNModelConfig {
    #[config(default = 64)]
    pub hidden_layer_1_size: usize,
    #[config(default = 96)]
    pub hidden_layer_2_size: usize,
    #[config(default = 64)]
    pub hidden_layer_3_size: usize,
    #[config(default = 0.99)]
    pub gamma: f32,
}

impl DQNModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> DQNModel<B> {
        let input_layer = LinearConfig::new(FEATRUE_SPACE, self.hidden_layer_1_size)
            .with_bias(true)
            .init(device);
        let hidden_layer_1 = LinearConfig::new(self.hidden_layer_1_size, self.hidden_layer_2_size)
            .with_bias(true)
            .init(device);
        let hidden_layer_2 = LinearConfig::new(self.hidden_layer_2_size, self.hidden_layer_3_size)
            .with_bias(true)
            .init(device);
        let output_layer = LinearConfig::new(self.hidden_layer_3_size, ACTION_SPACE)
            .with_bias(true)
            .init(device);

        DQNModel {
            input_layer,
            hidden_layer_1,
            hidden_layer_2,
            output_layer,
            loss_function: HuberLossConfig::new(1.34).init(device),
            activation: Relu::new(),
            gamma: self.gamma,
        }
    }
}

impl<B: Backend> DQNModel<B> {
    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = input.detach();
        let x = self.input_layer.forward(x);
        let x = self.activation.forward(x);
        self.output_layer.forward(x)
    }

    pub fn forward_step(&self, item: TankBatch<B>) -> DQNOutput<B> {
        let estimated_reward = self.forward(item.new_state);

        // FIXME: magic unsqueeze
        let a = item.action.unsqueeze_dim(1);
        let x = estimated_reward.clone().gather(1, a);

        // FIXME: what's final mask
        let expected_reward = self.forward(item.old_state);
        let y = expected_reward.clone().max_dim(1);
        let y = y.mul_scalar(self.gamma).add(item.reward.unsqueeze());

        let loss = self.loss_function.forward(x, y, Mean);

        DQNOutput {
            estimated_reward,
            expected_reward,
            loss,
        }
    }
}

impl<B: AutodiffBackend> TrainStep<TankBatch<B>, DQNOutput<B>> for DQNModel<B> {
    fn step(&self, item: TankBatch<B>) -> TrainOutput<DQNOutput<B>> {
        let item = self.forward_step(item);

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<TankBatch<B>, DQNOutput<B>> for DQNModel<B> {
    fn step(&self, item: TankBatch<B>) -> DQNOutput<B> {
        self.forward_step(item)
    }
}
