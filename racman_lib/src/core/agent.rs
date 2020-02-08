use crate::core::coordinate::Coord;
use std::any::Any;
use crate::{AgentImpl, RgbColor};

pub trait Agent {
    fn decide(&mut self, neighbors: &[AgentImpl]);
    fn coordinates(&self) -> Coord;
    fn direction(&self) -> Coord;
    fn set_direction(&mut self, dir: Coord);
    fn color(&self) -> &RgbColor;
    fn set_color(&mut self, color: RgbColor);
    fn as_any(&self) -> &dyn Any;
}