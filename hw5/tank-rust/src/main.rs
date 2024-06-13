mod dqn;
mod ffi;
use burn::backend::{wgpu::AutoGraphicsApi, Autodiff, Wgpu};

static ARTIFACT_DIR: &str = "../output";

type Backend = Wgpu<AutoGraphicsApi, f32, i32>;
type AutodiffBackend = Autodiff<Backend>;

pub fn main() {
    let device = burn::backend::wgpu::WgpuDevice::default();

    dqn::prelude::train::<AutodiffBackend>(device);
}
