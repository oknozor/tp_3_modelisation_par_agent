use crate::core::agent::Agent;
use crate::core::coordinate::Coord;
use crate::{AgentImpl, RgbColor};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Wall {
    pub coordinates: Coord,
}

impl Wall {
    pub fn new(coordinates: Coord) -> Wall {
        Wall { coordinates }
    }
}

const COLOR: RgbColor = RgbColor(0.0, 0.0, 0.0);

impl Agent for Wall {
    fn decide(&mut self, neighbors: &[AgentImpl]) {
        ()
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn direction(&self) -> Coord {
        Coord(0, 0)
    }

    fn set_direction(&mut self, dir: Coord) {
        ()
    }

    fn color(&self) -> &RgbColor {
        &COLOR
    }

    fn set_color(&mut self, color: RgbColor) {
        ()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
