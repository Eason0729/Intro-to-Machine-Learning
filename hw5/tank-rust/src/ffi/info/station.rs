#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
pub struct Station {
    pub x: i32,
    pub y: i32,
    pub power: i32,
}
