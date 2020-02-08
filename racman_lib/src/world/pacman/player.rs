use crate::core::agent::{Agent};
use crate::core::coordinate::Coord;
use std::any::Any;
use crate::{AgentImpl, RgbColor};

const PLAYER_COLOR: RgbColor = RgbColor(0.0, 0.0, 1.0);

pub struct Player {
    coordinates: Coord,
    direction: Coord,
}

impl Agent for Player {
    fn decide(&mut self, neighbors: &Vec<AgentImpl>) {
        let forward_position = self.get_forward_position();
        if let Some(_) = neighbors.iter().find(|agent| agent.borrow().coordinates() == forward_position) {
        } else {
            self.coordinates = forward_position
        }
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn color(&self) -> &RgbColor {
        &PLAYER_COLOR
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            coordinates: Coord(0,0),
            direction: Coord(0,0),
        }
    }

    fn get_forward_position(&self) -> Coord {
        self.coordinates + self.direction
    }
}