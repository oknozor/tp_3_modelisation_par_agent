use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::core::constants;
use crate::core::constants::{
    EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST, SOUTH_WEST, WEST,
};
use crate::core::coordinate::Coord;
use crate::world::pacman::wall::Wall;
use std::collections::HashSet;
use std::env::{current_exe, temp_dir};

pub fn gen() -> Vec<Wall> {
    let mut cells: Vec<bool> = (0..constants::size())
        .map(|i| true) // wall
        .collect();

    // clear the top row
    for i in 0..constants::max_width() as usize {
        cells[i] = false;
    }

    let mut rng = thread_rng();
    let height = constants::max_height();
    let width = constants::max_width();

    let mut current = height;
    let mut set = vec![current];

    let mut terminated = true;
    while terminated {
        let mut carve = rng.gen_range(0, 2);
        while carve == 1 || set.len() == 0 {
            current += 1;
            cells[current as usize] = false;
            set.push(current);
            carve = rng.gen_range(0, 2);
        }

        let idx = rng.gen_range(0, set.len());
        if set[idx] + width < constants::size() {
            let north = set[idx] + width;
            cells[north as usize] = false;
            set = vec![];
            current = current + 2;
        } else {
            terminated = false;
        };
    }

    cells
        .iter()
        .enumerate()
        .filter(|(i, cell)| **cell)
        .map(|(i, _)| Wall::new(Coord::from_idx(i as i32)))
        .collect()
}
