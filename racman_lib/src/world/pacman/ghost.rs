use std::any::Any;

use crate::{AgentImpl, RgbColor};
use crate::core::agent::Agent;
use crate::core::coordinate::Coord;

const GHOST_COLOR: RgbColor = RgbColor(0.0, 0.0, 0.0);

pub struct Ghost {
    pub coordinates: Coord,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost {
            coordinates: Coord(2, 3),
        }
    }
}

impl Agent for Ghost {
    fn decide(&mut self, neighbors: &Vec<AgentImpl>) {
        let forward_position = Coord(0, 1);
        if let Some(_) = neighbors.iter().find(|agent| agent.borrow().coordinates() == forward_position) {} else {
            self.coordinates = forward_position
        }
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn color(&self) -> &RgbColor {
        &GHOST_COLOR
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}