mod action;
mod agent;
mod huber;
mod state;

use smol::block_on;

use crate::data::prelude::*;

use self::agent::AIAgent;

pub struct TickState {
    pub frame: u64,
    pub player: Player,
    pub opponent: Opponent,
    pub foods: Vec<Food>,
}

struct AppState {}

pub struct App {
    state: AppState,
    agent: AIAgent,
}

impl App {
    pub fn new() -> Self {
        let agent = block_on(AIAgent::new());
        Self {
            state: AppState {},
            agent,
        }
    }
    pub fn run(&mut self, tick: TickState) -> Direction {
        self.agent.tick(tick.into()).into()
    }
    pub fn check_point(&mut self) {
        self.agent.check_point();
    }
}
