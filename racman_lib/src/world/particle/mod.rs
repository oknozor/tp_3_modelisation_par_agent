use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;

use crate::{AgentImpl, RgbColor};
use crate::core::agent::Agent;
use crate::core::constants;
use crate::core::coordinate::Coord;

pub struct Particle {
    coordinates: Coord,
    color: RgbColor,
    direction: Coord,
}

impl Particle {
    pub fn new(coordinates: Coord) -> Particle {
        Particle {
            coordinates,
            direction: Coord::random_dir(),
            color: RgbColor::black(),
        }
    }

    pub fn collide(&mut self) {
        self.direction.invert_direction();
        self.set_color(RgbColor::red());
    }
}

impl Agent for Particle {
    fn decide(&mut self, neighbors: &[AgentImpl]) {
        let mut agent_collision = false;
        let next_position = self.coordinates + self.direction;

        for agent_ref in neighbors {
            let borrowed: &RefCell<dyn Agent> = agent_ref.borrow();
            let mut ref_mut = borrowed.borrow_mut();
            if next_position == ref_mut.coordinates() {
                ref_mut.set_direction(self.direction);
                ref_mut.set_color(RgbColor::red());
                agent_collision = true;
                break;
            }
        }

        if agent_collision {
            self.collide()
        } else {
            match (self.coordinates, self.direction) {
                (Coord(x, _), constants::EAST) if x == constants::max_width() - 1 => self.collide(),
                (Coord(x, _), constants::WEST) if x == 0 => self.collide(),
                (Coord(_, y), constants::NORTH) if y == 0 => self.collide(),
                (Coord(_, y), constants::SOUTH) if y == constants::max_height() - 1 => self.collide(),
                (Coord(x, y), constants::SOUTH_EAST) if y == constants::max_height() - 1 || x == constants::max_width() - 1 => self.collide(),
                (Coord(x, y), constants::SOUTH_WEST) if y == constants::max_height() - 1 || x == 0 => self.collide(),
                (Coord(x, y), constants::NORTH_EAST) if y == 0 || x == constants::max_width() - 1 => self.collide(),
                (Coord(x, y), constants::NORTH_WEST) if y == 0 || x == 0 => self.collide(),
                _ => self.coordinates = self.coordinates + self.direction
            }
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
        &self.color
    }

    fn set_color(&mut self, color: RgbColor) {
        self.color = color;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}
