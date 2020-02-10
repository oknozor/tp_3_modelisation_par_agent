use crate::core::agent::{Agent, AgentKind};
use crate::core::coordinate::Coord;
use crate::{AgentImpl, RgbColor};

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
    fn decide(&mut self, _: &[AgentImpl]) {
        ()
    }

    fn coordinates(&self) -> Coord {
        self.coordinates
    }

    fn direction(&self) -> Coord {
        Coord(0, 0)
    }

    fn set_direction(&mut self, _: Coord) {
        ()
    }

    fn color(&self) -> &RgbColor {
        &COLOR
    }

    fn set_color(&mut self, _: RgbColor) {
        ()
    }

    fn get_kind(&self) -> AgentKind {
        AgentKind::Wall
    }
}
