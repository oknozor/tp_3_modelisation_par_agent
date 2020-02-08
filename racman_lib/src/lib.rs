#![feature(box_syntax)]

pub mod core;
pub mod world;

use std::cell::RefCell;
use std::rc::Rc;
use crate::core::agent::Agent;

pub type AgentImpl = Rc<RefCell<dyn Agent>>;

pub struct RgbColor(pub f32, pub f32, pub f32);

impl RgbColor {
    pub fn red() -> RgbColor {
        RgbColor(1.0, 0.0, 0.0)
    }

    pub fn blue() -> RgbColor {
        RgbColor(0.0, 0.0, 1.0)
    }

    pub fn black() -> RgbColor {
        RgbColor(0.0, 0.0, 0.0)
    }
}

