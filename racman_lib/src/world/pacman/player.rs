use crate::core::agent::{Agent};
use crate::core::coordinate::Coord;
use std::any::Any;
use crate::{AgentImpl, RgbColor};

const PLAYER_COLOR: RgbColor = RgbColor(0.0, 0.0, 1.0);

pub struct Player {
    coordinates: Coord,
    direction: Coord,
    color: RgbColor
}

impl Agent for Player {
    fn decide(&mut self, neighbors: &[AgentImpl]) {
        let forward_position = self.get_forward_position();
        if let Some(_) = neighbors.iter().find(|agent| agent.borrow().coordinates() == forward_position) {
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
        &PLAYER_COLOR
    }

    fn set_color(&mut self, color: RgbColor) {
        self.color = color;
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
            color: RgbColor::blue(),
        }
    }

    fn get_forward_position(&self) -> Coord {
        self.coordinates + self.direction
    }
}