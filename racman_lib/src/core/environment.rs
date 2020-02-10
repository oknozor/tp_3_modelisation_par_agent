use crate::core::agent::{Agent, AgentKind};
use crate::core::constants::{
    set_max_height, set_max_width, EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST,
    SOUTH_WEST, WEST,
};
use crate::core::coordinate::Coord;
use crate::AgentImpl;
use std::borrow::Borrow;
use std::cell::RefCell;

pub struct Environment {
    pub agents: Vec<AgentImpl>,
}

impl Environment {
    pub fn new(width: i32, height: i32) -> Environment {
        set_max_height(height);
        set_max_width(width);

        Environment { agents: vec![] }
    }

    pub fn tick(&mut self) {
        for idx in 0..self.agents.len() {
            let mut ref_mut = self.agents[idx].borrow_mut();
            let coord = ref_mut.coordinates();
            let kind = ref_mut.get_kind();

            let neighbors = match kind {
                AgentKind::Player => self.get_neighbors(idx, coord),
                AgentKind::Ghost => {
                    [
                        &self.agents[0..idx],
                        &self.agents[idx + 1..self.agents.len()]
                    ].concat()
                },
                AgentKind::Particle => self.get_neighbors(idx, coord),
                AgentKind::Wall => return,
                AgentKind::Fish => return,
                AgentKind::Shark => return,
            };
            ref_mut.decide(&neighbors);
        }
    }

    pub(crate) fn add_agent(&mut self, agent: AgentImpl) {
        self.agents.push(agent);
    }

    pub fn get_neighbors(&self, idx: usize, coord: Coord) -> Vec<AgentImpl> {
        let mut neighbors = vec![];
        for i in 0..self.agents.len() {
            if i != idx {
                let agent: &RefCell<dyn Agent> = self.agents[i].borrow();
                let n_coordinate: Coord = agent.borrow().coordinates();
                if n_coordinate == coord + NORTH
                    || n_coordinate == coord + SOUTH
                    || n_coordinate == coord + EAST
                    || n_coordinate == coord + WEST
                    || n_coordinate == coord + SOUTH_EAST
                    || n_coordinate == coord + SOUTH_WEST
                    || n_coordinate == coord + NORTH_WEST
                    || n_coordinate == coord + NORTH_EAST
                {
                    neighbors.push(self.agents[i].clone());
                }
            } else {
                continue;
            };
        }
        neighbors
    }
}

// Pacman
impl Environment {
    pub fn update_player_direction(&mut self, direction: Coord) {
        self.agents[0].borrow_mut().set_direction(direction);
    }
}
