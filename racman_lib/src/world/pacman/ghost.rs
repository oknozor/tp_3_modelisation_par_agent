use std::any::Any;

use crate::core::agent::Agent;
use crate::core::coordinate::Coord;
use crate::{AgentImpl, RgbColor};

const GHOST_COLOR: RgbColor = RgbColor(0.0, 0.0, 0.0);

pub struct Ghost {
    pub coordinates: Coord,
    pub direction: Coord,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost {
            coordinates: Coord(2, 3),
            direction: Coord::random_dir(),
        }
    }
}

impl Agent for Ghost {
    fn decide(&mut self, neighbors: &[AgentImpl]) {
        let forward_position = Coord(0, 1);
        if let Some(_) = neighbors
            .iter()
            .find(|agent| agent.borrow().coordinates() == forward_position)
        {
        } else {
            self.coordinates = forward_position
        }
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn direction(&self) -> Coord {
        self.direction
    }

    fn set_direction(&mut self, dir: Coord) {
        self.direction = dir
    }

    fn color(&self) -> &RgbColor {
        &GHOST_COLOR
    }

    fn set_color(&mut self, color: RgbColor) {
        //        self.color = color;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
