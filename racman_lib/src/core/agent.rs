use crate::core::coordinate::Coord;
use crate::{AgentImpl, RgbColor};

#[derive(Eq, PartialEq)]
pub enum AgentKind {
    Player,
    Wall,
    Ghost,
    Fish,
    Shark,
    Particle,
}

pub trait Agent {
    fn decide(&mut self, neighbors: &[AgentImpl]);
    fn coordinates(&self) -> Coord;
    fn direction(&self) -> Coord;
    fn set_direction(&mut self, dir: Coord);
    fn color(&self) -> &RgbColor;
    fn set_color(&mut self, color: RgbColor);
    fn get_kind(&self) -> AgentKind;
}
