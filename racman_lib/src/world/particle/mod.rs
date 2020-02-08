use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

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
}

impl Agent for Particle {
    fn decide(&mut self, neighbors: &Vec<AgentImpl>) {
        let mut agent_collision = false;
        let next_position = self.coordinates + self.direction;

        for agent_ref in neighbors {
            let downcast = Rc::clone(&agent_ref);
            let downcast: &RefCell<dyn Agent> = downcast.borrow();
            let downcast = downcast.clone().borrow();
            let any = downcast.as_any();
            let neighbor = any.downcast_ref::<Particle>();

            let neighbor = neighbor.unwrap();
            neighbor.color = RgbColor::red();

            if next_position == neighbor.coordinates  {
                neighbor.direction.invert_direction();
                agent_collision = true;
                break;
            }
        }

        if agent_collision {
            self.color = RgbColor::red();
            self.direction.invert_direction();
        } else {
            match (self.coordinates, self.direction) {
                (Coord(x, _), constants::EAST) if x == constants::max_width() - 1 => self.direction.invert_direction(),
                (Coord(x, _), constants::WEST) if x == 0 => self.direction.invert_direction(),
                (Coord(_, y), constants::NORTH) if y == 0 => self.direction.invert_direction(),
                (Coord(_, y), constants::SOUTH) if y == constants::max_height() - 1 => self.direction.invert_direction(),
                (Coord(x, y), constants::SOUTH_EAST) if y == constants::max_height() - 1 || x == constants::max_width() - 1 => self.direction.invert_direction(),
                (Coord(x, y), constants::SOUTH_WEST) if y == constants::max_height() - 1 || x == 0 => self.direction.invert_direction(),
                (Coord(x, y), constants::NORTH_EAST) if y == 0 - 1 || x == constants::max_width() - 1 => self.direction.invert_direction(),
                (Coord(x, y), constants::NORTH_WEST) if y == 0 - 1 || x == 0 => self.direction.invert_direction(),
                _ => self.coordinates = self.coordinates + self.direction
            }
        }
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn color(&self) -> &RgbColor {
        &self.color
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
