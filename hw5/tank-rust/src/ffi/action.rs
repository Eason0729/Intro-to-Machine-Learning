#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Action {
    Forward = 0,
    Backward = 1,
    TurnRight = 2,
    TurnLeft = 3,
    AimRight = 4,
    AimLeft = 5,
    Shoot = 6,
    None = 7,
}

// impl TryFrom<i32> for Action {
//     type Error = &'static str;

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Action::Forward),
//             1 => Ok(Action::Backward),
//             2 => Ok(Action::TurnRight),
//             3 => Ok(Action::TurnLeft),
//             4 => Ok(Action::AimRight),
//             5 => Ok(Action::AimLeft),
//             6 => Ok(Action::Shoot),
//             7 => Ok(Action::None),
//             _ => Err("Invalid action"),
//         }
//     }
// }
