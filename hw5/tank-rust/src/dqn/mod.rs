mod collect;
mod dataset;
mod feature;
mod model;
mod training;

pub mod prelude {
    pub use super::collect::App as DQNApp;
    pub use super::dataset::{TankBatcher, TankDataset, TankItem};
    pub use super::feature::{ACTION_SPACE, FEATRUE_SPACE};
    pub use super::training::{run as train, ExpConfig};
}
