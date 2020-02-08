use crate::core::coordinate::Coord;
use std::any::Any;
use crate::{AgentImpl, RgbColor};

pub trait Agent {
    fn decide(&mut self, neighbors: &Vec<AgentImpl>);
    fn coordinates(&self) -> Coord;
    fn color(&self) -> &RgbColor;
    fn as_any(&self) -> &dyn Any;
}