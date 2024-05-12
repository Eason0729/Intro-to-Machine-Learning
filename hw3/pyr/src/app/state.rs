use candle_core::{Device, Tensor};

use crate::{Food, Opponent, Player};

use super::TickState;

pub const OBSERVATION_SPACE: usize = 14;

#[derive(Clone)]
pub struct AIState {
    pub frame: u64,
    pub player: Player,
    pub opponent: Opponent,
    pub foods: Vec<Food>,
}

impl From<TickState> for AIState {
    fn from(value: TickState) -> Self {
        Self {
            player: value.player,
            opponent: value.opponent,
            foods: value.foods,
            frame: value.frame,
        }
    }
}

fn food_distance<'a>(player: &'a Player) -> impl FnMut(&&Food) -> i32 + 'a {
    move |food: &&Food| {
        let dx = player.x - food.x;
        let dy = player.y - food.y;
        ((dx.abs() + dy.abs()) * 100.0) as i32
    }
}
impl AIState {
    pub fn get_postivie_food(&self) -> Option<&Food> {
        self.foods
            .iter()
            .filter(|x| x.score.is_sign_positive())
            .min_by_key(food_distance(&self.player))
    }
    pub fn get_negative_food(&self) -> Option<&Food> {
        self.foods
            .iter()
            .filter(|x| x.score.is_sign_negative())
            .min_by_key(food_distance(&self.player))
    }
    pub fn into_tensor(&self, device:&Device) -> Tensor {
        Tensor::new(&[self.into_feature()],device).unwrap()
    }
    fn into_feature(&self) -> [f32; OBSERVATION_SPACE] {
        let x = self.player.x;
        let y = self.player.y;
        // sort food into four group by two line (x+y=0, x-y=0)
        let mut food_group = [
            0.0,
            0.0,
            0.0,
            0.0,
            self.opponent.x - self.player.x / 700.0,
            self.opponent.y - self.player.y / 700.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        for food in self.foods.iter().filter(|x| x.score.is_sign_positive()) {
            let dx = food.x - x;
            let dy = food.y - y;
            let group = match (dx + dy, dx - dy) {
                (a, b) if a.is_sign_positive() && b.is_sign_positive() => 0,
                (a, b) if a.is_sign_positive() && b.is_sign_positive() => 1,
                (a, b) if a.is_sign_negative() && b.is_sign_negative() => 2,
                _ => 3,
            };
            food_group[group] += 10.0 / (dx + dy);
        }
        for food in self.foods.iter().filter(|x| x.score.is_sign_negative()) {
            let dx = food.x - x;
            let dy = food.y - y;
            let group = match (dx + dy, dx - dy) {
                (a, b) if a.is_sign_positive() && b.is_sign_positive() => 6,
                (a, b) if a.is_sign_positive() && b.is_sign_positive() => 7,
                (a, b) if a.is_sign_negative() && b.is_sign_negative() => 8,
                _ => 9,
            };
            food_group[group] += 10.0 / (dx + dy);
        }
        self.get_postivie_food().map(|food| {
            let dx = food.x - x;
            let dy = food.y - y;
            food_group[10] = dx as f32;
            food_group[11] = dy as f32;
        });
        self.get_negative_food().map(|food| {
            let dx = food.x - x;
            let dy = food.y - y;
            food_group[12] = dx as f32;
            food_group[13] = dy as f32;
        });


        food_group
    }
}
