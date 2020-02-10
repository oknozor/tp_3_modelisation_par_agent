use crate::core::constants::{set_max_height, set_max_width, NORTH, SOUTH, WEST, EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST, NORTH_EAST};
use crate::core::coordinate::Coord;
use crate::AgentImpl;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use crate::core::agent::Agent;

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

            let neighbors = &self.get_neighbors(idx, coord);
            ref_mut.decide(neighbors);
        }
    }

    pub(crate) fn add_agent(&mut self, agent: AgentImpl) {
        self.agents.push(agent);
    }

    fn remove_agent(&mut self, address: Coord) {
        let _ = self.agents.remove(address.as_idx());
    }

    // returns closest agents regardless of the distance
    pub fn get_neighbors(&self, idx: usize, coord: Coord) -> Vec<AgentImpl> {
        let mut neighbors = vec![];
        for i in 0..self.agents.len() {
            if i != idx {
                let agent: &RefCell<dyn Agent> = self.agents[i].borrow();
                let n_coordinate: Coord = agent.borrow().coordinates();
                if n_coordinate == coord + NORTH ||
                    n_coordinate == coord + SOUTH ||
                    n_coordinate == coord + EAST ||
                    n_coordinate == coord + WEST ||
                    n_coordinate == coord + SOUTH_EAST ||
                    n_coordinate == coord + SOUTH_WEST ||
                    n_coordinate == coord + NORTH_WEST ||
                    n_coordinate == coord + NORTH_EAST {
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
