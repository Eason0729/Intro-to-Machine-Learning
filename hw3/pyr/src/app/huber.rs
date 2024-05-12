use candle_core::{Tensor, WithDType};

pub trait Half
where
    Self: WithDType + Copy,
{
    const HALF: Self;
}

impl Half for f64 {
    const HALF: f64 = 0.5;
}
impl Half for f32 {
    const HALF: f32 = 0.5;
}

pub fn huber_loss<D: WithDType + Half>(threshold: D) -> impl Fn(&Tensor, &Tensor) -> D {
    move |x: &Tensor, y: &Tensor| {
        let diff = (x - y).unwrap();
        let diff_scaler = diff
            .abs()
            .unwrap()
            .sum_all()
            .unwrap()
            .to_scalar::<D>()
            .unwrap();
        match diff_scaler < threshold {
            true => <D as Half>::HALF * diff_scaler,
            false => threshold * (diff_scaler - <D as Half>::HALF * threshold),
        }
    }
}
