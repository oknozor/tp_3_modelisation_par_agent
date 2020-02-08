use crate::core::constants::{set_max_height, set_max_width};
use crate::core::coordinate::Coord;
use crate::AgentImpl;

pub struct Environment {
    pub agents: Vec<AgentImpl>
}

impl Environment {
    pub fn new(width: i32, height: i32) -> Environment {
        set_max_height(height);
        set_max_width(width);

        Environment {
            agents: vec![]
        }
    }

    pub fn tick(&mut self) {
        for idx in 0..self.agents.len() {
            let neighbors = &self.get_neighbors(idx);

            self.agents[idx].borrow_mut().decide(neighbors);
        }
    }

    pub(crate) fn add_agent(&mut self, agent: AgentImpl) {
        self.agents.push(agent);
    }

    fn remove_agent(&mut self, address: Coord) {
        let _ = self.agents.remove(address.as_idx());
    }

    // returns closest agents regardless of the distance
    pub fn get_neighbors(&self, idx: usize) -> Vec<AgentImpl> {
        let agent_size = self.agents.len() - 1;

        let start = if idx as i32 - 4 < 0 {
            0
        } else {
            idx - 4
        };

        let left = self.agents[start..idx].to_vec();

        let end = if idx + 4 > agent_size {
            agent_size
        } else {
            idx  as usize + 4
        };

        let right = if idx == agent_size {
            vec![]
        } else {
            self.agents[idx as usize + 1..end].to_vec()
        };

        [left, right].concat().to_vec()
    }
}

// Pacman
impl Environment {
    pub fn update_player_direction(&mut self, direction: Coord) {
        self.agents[0].borrow_mut().set_direction(direction);
    }
}
