use crate::Direction;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum AIAction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl From<AIAction> for Direction {
    fn from(value: AIAction) -> Self {
        match value {
            AIAction::Up => Direction::Up,
            AIAction::Down => Direction::Down,
            AIAction::Left => Direction::Left,
            AIAction::Right => Direction::Right,
            AIAction::None => Direction::None,
        }
    }
}
