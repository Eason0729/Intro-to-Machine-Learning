use super::internal::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawOverall {
    pub frame: u64,
    score: i64,
    score_to_pass: i64,
    self_x: i64,
    self_y: i64,
    self_h: i64,
    self_w: i64,
    self_vel: i64,
    self_lv: i64,
    opponent_x: i64,
    opponent_y: i64,
    opponent_lv: i64,
}

impl RawOverall {
    pub fn get_player(&self) -> Player {
        Player {
            x: (self.self_x - 350) as f32,
            y: (self.self_y - 350) as f32,
            height: self.self_h as f32,
            width: self.self_w as f32,
            level: self.self_lv as f32,
            velocity: self.self_vel as f32,
            score: self.score as f32,
        }
    }
    pub fn get_opponent(&self) -> Opponent {
        Opponent {
            x: (self.opponent_x - 350) as f32,
            y: (self.opponent_y - 350) as f32,
            level: self.opponent_lv as f32,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RawFood {
    pub h: i64,
    pub w: i64,
    pub x: i64,
    pub y: i64,
    pub score: i64,
    pub kind: i32,
}

impl From<RawFood> for Food {
    fn from(value: RawFood) -> Self {
        Food {
            x: value.x as f32,
            y: value.y as f32,
            width: value.w as f32,
            height: value.h as f32,
            score: value.score as f32,
        }
    }
}

#[repr(i32)]
#[derive(Debug)]
pub enum FoodKind {
    Food1 = 1,
    Food2 = 2,
    Food3 = 3,
    Garbage1 = 4,
    Garbage2 = 5,
    Garbage3 = 6,
}

#[repr(i32)]
pub enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    None = 5,
}
