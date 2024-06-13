#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub lives: i32,
}
