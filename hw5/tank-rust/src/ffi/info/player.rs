#[repr(C)]
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub score: i32,
    pub power: i32,
    pub oil: f32,
    pub lives: i32,
    pub angle: i32,
    pub gun_angle: i32,
    pub cooldown: i32,
}
