use crate::core::agent::{Agent, AgentKind};
use crate::core::constants;
use crate::core::coordinate::Coord;
use crate::{AgentImpl, RgbColor};

const GHOST_COLOR: RgbColor = RgbColor(1.0, 0.0, 0.0);

pub struct Ghost {
    pub coordinates: Coord,
    pub direction: Coord,
}

impl Ghost {
    pub fn new(coord: Coord) -> Ghost {
        Ghost {
            coordinates: coord,
            direction: Coord::random_dir(),
        }
    }
}

impl Agent for Ghost {
    fn decide(&mut self, neighbors: &[AgentImpl]) {
        let target = neighbors[0].clone();

        // All the cells
        let mut cells: Vec<(bool, Coord, i32)> = (0..constants::size())
            .map(|i| (true, Coord::from_idx(i), i32::max_value())) // (empty, coord, dist,)
            .collect();

        // map to (is_empty, coord, MAX_i32)
        neighbors
            .iter()
            .map(|agent| agent.borrow().coordinates())
            .for_each(|coord| cells[coord.as_idx()] = (false, coord, i32::max_value()));

        let mut dist = 0;
        let player_idx = neighbors[0].borrow().coordinates().as_idx();
        let target = self.coordinates().as_idx();

        // mark self idx as occupied
        cells[target] = (false, self.coordinates, 0);
//
//        let mut terminated = false;
//        let mut current_idx = player_idx;
//        while !terminated {
//            let previous = (&mut cells, dist, player_idx, current_idx);
//            terminated = dist == previous;
//        }
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
        &GHOST_COLOR
    }

    fn set_color(&mut self, _: RgbColor) {
        ()
    }

    fn get_kind(&self) -> AgentKind {
        AgentKind::Ghost
    }
}

impl Ghost {
//    fn explore(mut cells: &mut Vec<(bool, Coord, i32)>, mut dist: i32, mut current_idx: usize) -> i32 {
//        let north = current_idx.north().as_idx();
//        let south = current_idx.south().as_idx();
//        let east = current_idx.east().as_idx();
//        let west = current_idx.west().as_idx();
//
//        Ghost::mark_cell(cells, dist, north);
//        Ghost::mark_cell(cells, dist, south);
//        Ghost::mark_cell(cells, dist, east);
//        Ghost::mark_cell(cells, dist, west);
//
//        let min = [cells[north], cells[south], cells[east], cells[west]]
//            .iter()
//            .filter(|cell| cell.0)
//            .map(|cell| cell.2).min;
//        Ghost::explore(cells, dist, north);
//        Ghost::explore(cells, dist, south);
//        Ghost::explore(cells, dist, east);
//        Ghost::explore(cells, dist, east);
//        dist
//    }
//
//    fn mark_cell(mut cells: &mut Vec<(bool, Coord, i32)>, dist: i32, current_idx: usize) {
//        let cell_value = cells[current_idx].2;
//        if cell_value > dist {
//            cells[current_idx].2 = dist + 1
//        }
//    }
}
