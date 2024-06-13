use burn::{
    data::dataloader::DataLoaderBuilder,
    optim::{AdamConfig, SgdConfig},
    record::{CompactRecorder, NoStdTrainingRecorder},
    tensor::backend::AutodiffBackend,
    train::{
        metric::{
            store::{Aggregate, Direction, Split},
            LossMetric,
        },
        LearnerBuilder, MetricEarlyStoppingStrategy, StoppingCondition,
    },
};

use crate::dqn::prelude::{ExpConfig, TankBatcher, TankDataset};

pub fn run<B: AutodiffBackend>(device: B::Device) {
    // let d = [
    //     feature[0],
    //     -feature[0],
    //     shoot_target_angle*0.7*feature[2],
    //     -shoot_target_angle*0.7*feature[2],
    //     8.0 * feature[2] / shoot_target_distance / shoot_target_angle,
    //     feature[2]*shoot_target_distance*0.3-feature[2],
    // ];

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

    // Define train/test datasets and dataloaders

    let train_dataset = TankDataset::train();
    let test_dataset = TankDataset::test();

    println!("Train Dataset Size: {}", train_dataset.len());
    println!("Test Dataset Size: {}", test_dataset.len());

    let batcher_train = TankBatcher::<B>::new(device.clone());

    let batcher_test = TankBatcher::<B::InnerBackend>::new(device.clone());

    // Since dataset size is small, we do full batch gradient descent and set batch size equivalent to size of dataset

    let dataloader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(train_dataset.len())
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(train_dataset);

    let dataloader_test = DataLoaderBuilder::new(batcher_test)
        .batch_size(test_dataset.len())
        .shuffle(config.seed)
        .num_workers(config.num_workers)
        .build(test_dataset);

    // Model
    let learner = LearnerBuilder::new(ARTIFACT_DIR)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(CompactRecorder::new())
        .early_stopping(MetricEarlyStoppingStrategy::new::<LossMetric<B>>(
            Aggregate::Mean,
            Direction::Lowest,
            Split::Valid,
            StoppingCondition::NoImprovementSince { n_epochs: 1 },
        ))
        .devices(vec![device.clone()])
        .num_epochs(config.num_epochs)
        .summary()
        .build(model, config.optimizer.init(), 5e-3);

    let model_trained = learner.fit(dataloader_train, dataloader_test);

    config
        .save(format!("{ARTIFACT_DIR}/config.json").as_str())
        .unwrap();

    model_trained
        .save_file(
            format!("{ARTIFACT_DIR}/model"),
            &NoStdTrainingRecorder::new(),
        )
        .expect("Failed to save trained model");
}
