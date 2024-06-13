use std::{env, fs};

use super::dataset::{TankBatcher, TankDataset};
use super::model::DQNModelConfig;
use crate::ARTIFACT_DIR;
use burn::optim::AdamConfig;
use burn::train::metric::store::{Aggregate, Direction, Split};
use burn::train::{MetricEarlyStoppingStrategy, StoppingCondition};
use burn::{
    data::{dataloader::DataLoaderBuilder, dataset::Dataset},
    prelude::*,
    record::{CompactRecorder, NoStdTrainingRecorder},
    tensor::backend::AutodiffBackend,
    train::{metric::LossMetric, LearnerBuilder},
};
#[derive(Config)]
pub struct ExpConfig {
    #[config(default = 16)]
    pub num_epochs: usize,

    #[config(default = 6)]
    pub num_workers: usize,

    #[config(default = 47)]
    pub seed: u64,

    pub optimizer: AdamConfig,

    #[config(default = 1.5e-3)]
    pub learn_rate: f64,

    #[config(default = 4096)]
    pub batch_size: usize,
}

pub fn run<B>(device: B::Device)
where
    B: AutodiffBackend,
{
    let model_path = env::var("MODEL_PATH").unwrap_or_else(|_| ARTIFACT_DIR.to_string());

    let optimizer = AdamConfig::new();
    let config = ExpConfig::new(optimizer);
    let mut model = DQNModelConfig::new().init(&device);

    if fs::metadata(format!("{model_path}/model")).is_ok() {
        model = model
            .load_file(
                format!("{model_path}/model"),
                &NoStdTrainingRecorder::new(),
                &device,
            )
            .unwrap();
    }

    B::seed(config.seed);

    let (train_dataset, test_dataset) = TankDataset::load().split(1.0);

    println!("Train Dataset Size: {}", train_dataset.len());
    println!("Test Dataset Size: {}", test_dataset.len());

    let batcher_train = TankBatcher::<B>::new(device.clone());

    let batcher_test = TankBatcher::<B::InnerBackend>::new(device.clone());

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(train_dataset);

    let dataloader_test = DataLoaderBuilder::new(batcher_test)
        .batch_size(config.batch_size)
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(test_dataset);

    let learner = LearnerBuilder::new(&model_path)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .early_stopping(MetricEarlyStoppingStrategy::new::<LossMetric<B>>(
            Aggregate::Mean,
            Direction::Lowest,
            Split::Train,
            StoppingCondition::NoImprovementSince { n_epochs: 2 },
        ))
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(model, config.optimizer.init(), config.learn_rate);

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    config
        .save(format!("{model_path}/config.json").as_str())
        .unwrap();

    model_trained
        .save_file(format!("{model_path}/model"), &NoStdTrainingRecorder::new())
        .expect("Failed to save trained model");
}
