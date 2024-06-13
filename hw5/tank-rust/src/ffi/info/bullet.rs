#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Bullet {
    pub x: i32,
    pub y: i32,
}
