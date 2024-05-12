#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub level: f32,
    pub velocity: f32,
    pub score: f32,
}
#[derive(Clone)]
pub struct Opponent {
    pub x: f32,
    pub y: f32,
    pub level: f32,
}

#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub score: f32,
}

impl Default for Food {
    fn default() -> Self {
        Food {
            x: 1000000.0,
            y: 1000000.0,
            width: 1.0,
            height: 1.0,
            score: 0.0,
        }
    }
}
