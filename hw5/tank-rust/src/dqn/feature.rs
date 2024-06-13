//! Feature extraction and reward calculation for DQN
use std::f32::consts::PI;

use burn::tensor::{backend::Backend, Tensor};

use crate::ffi::prelude::*;

pub const FEATRUE_SPACE: usize = 7;
pub const ACTION_SPACE: usize = 6;

#[derive(PartialEq, Default)]
struct Polar {
    angle: f32,
    distance: f32,
}

impl Polar {
    pub fn clip(&self) -> Self {
        Polar {
            angle: self.angle,
            distance: self.distance.max(0.0).min(1e3),
        }
    }
}

impl Eq for Polar {}

impl Ord for Polar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl PartialOrd for Polar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

fn normalize_angle(mut angle: f32) -> f32 {
    while angle < -PI {
        angle += 2.0 * PI;
    }
    while angle >= PI {
        angle -= 2.0 * PI;
    }
    angle
}

impl Player {
    fn to_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn center(&self, x: i32, y: i32) -> Polar {
        let dx = x - self.x;
        let dy = y - self.y;
        let angle = (dy as f32).atan2(dx as f32);
        let distance = (dx.pow(2) + dy.pow(2)) as f32;
        Polar { angle, distance }
    }
    fn closest(&self, others: impl Iterator<Item = (i32, i32)>) -> Polar {
        others
            .map(|(x, y)| self.center(x, y))
            .min()
            .unwrap_or_default()
    }
    fn get_angle(&self) -> f32 {
        (180 - self.angle) as f32 / 360.0 * 2.0 * PI
    }
    fn get_gun_angle(&self) -> f32 {
        self.gun_angle as f32 / 360.0 * 2.0 * PI
    }
}

#[derive(Debug)]
enum Target {
    Oil,
    Bullet,
    Enemy,
}

impl Target {
    fn get_pos(&self, info: &Info) -> Polar {
        match self {
            Target::Oil => info
                .player
                .closest(info.oil_stations.iter().map(Station::to_pos)),
            Target::Bullet => info
                .player
                .closest(info.bullet_stations.iter().map(Station::to_pos)),
            Target::Enemy => info.player.closest(info.enemies.iter().map(Player::to_pos)),
        }
    }
    fn reach(&self, last: &Info, current: &Info) -> bool {
        match self {
            Target::Oil => last.player.oil > current.player.oil,
            Target::Bullet => last.player.power > current.player.power,
            Target::Enemy => false,
        }
    }
}

impl Station {
    fn to_pos(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

impl Wall {
    fn to_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl<'a> Info<'a> {
    pub fn into_feature(&self) -> [f32; FEATRUE_SPACE] {
        let emeny = self.player.closest(self.enemies.iter().map(Player::to_pos));
        let wall = self
            .player
            .closest(self.walls.iter().map(|wall| (wall.x, wall.y)));

        let target = self.get_target().get_pos(self).clip();

        let angle = self.player.get_angle();
        let gun_angle = self.player.get_gun_angle();

        [
            normalize_angle(target.angle - angle).tanh(),
            (wall.distance - target.distance).tanh(),
            (self.player.power as f32).tanh(),
            (wall.clip().distance + 1.0).log2(),
            (emeny.distance + 1.0).log2(),
            normalize_angle(emeny.angle - gun_angle).tanh(),
            normalize_angle(wall.angle - gun_angle).tanh(),
        ]
    }
    pub fn into_feature_tensor<B: Backend>(&self, device: &B::Device) -> Tensor<B, 1> {
        let feature = self.into_feature();

        Tensor::from_floats(feature, device)
    }
    fn get_target(&self) -> Target {
        if self.player.oil < 40.0 {
            Target::Oil
        } else if self.player.power > 7 {
            Target::Enemy
        } else {
            Target::Bullet
        }
    }
    pub fn get_reward(&self, next: &Self, action: Action) -> f32 {
        let same_position = self.player.x == next.player.x && self.player.y == next.player.y;
        let mut reward = -2.3;
        reward += match action {
            Action::Forward | Action::Backward if same_position => -8.0,
            Action::Shoot => match next.player.power > 7 {
                true => 2.0,
                false => -2.0,
            },
            _ => 0.0,
        };

        let target = self.get_target();

        if target.reach(self, next) {
            reward += 15.0;
        } else {
            let previous_target_position = target.get_pos(self);
            let next_target_position = target.get_pos(next);

            reward += match previous_target_position.cmp(&next_target_position) {
                std::cmp::Ordering::Less => -5.0,
                std::cmp::Ordering::Equal => 0.0,
                std::cmp::Ordering::Greater => 5.8,
            };
        }

        reward
            + match next.player.score - self.player.score {
                x if x > 2 => 20.0, // bypass emeny
                x if x > 0 => 10.0, // too high, tank my ignore power station
                _ => -1.0,
            }
    }
}
