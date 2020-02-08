use std::ops;
use std::cmp::Ordering;
use crate::core::constants::{max_height, max_width};
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    pub fn as_idx(&self) -> usize {
        (self.1 * max_width() + self.0) as usize
    }

    pub fn from_idx(idx: i32) -> Coord {
        let x = idx % max_width();
        let y = (idx - x) / max_height();
        Coord(x, y)
    }

    pub fn invert_direction(&mut self) {
        match self.0 {
            1 => self.0 = -1,
            -1 => self.0 = 1,
            0 => (),
            _ => panic!("Invalid x direction value")
        }
        match self.1 {
            1 => self.1 = -1,
            -1 => self.1 = 1,
            0 => (),
            _ => panic!("Invalid v direction value")
        }
    }

    pub fn random_dir() -> Coord {
        use crate::core::constants::*;
        let dirs = vec![EAST, WEST, NORTH, SOUTH, SOUTH_WEST, SOUTH_EAST, NORTH_WEST, NORTH_EAST];
        let rng = thread_rng().gen_range(0, 8);
        dirs[rng]
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        let res = Coord(self.0 + rhs.0, self.1 + rhs.1);
        if res.0 < 0 || res.0 >= max_width() || res.1 < 0 || res.1 >= max_height() {
            self
        } else {
            res
        }
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_idx().cmp(&other.as_idx())
    }
}

#[cfg(test)]
mod test {
    use crate::core::coordinate::Coord;
    use crate::core::constants::{set_max_width, set_max_height};

    #[test]
    fn should_add() {
        let lhs = Coord(0, 0);
        let rhs = Coord(0, 1);
        set_max_width(10);
        set_max_height(10);


        assert_eq!(lhs + rhs, Coord(0, 1));

        let lhs = Coord(0, 1);
        let rhs = Coord(0, 1);

        assert_eq!(lhs + rhs, Coord(0, 2));

        let lhs = Coord(1, 0);
        let rhs = Coord(0, 1);

        assert_eq!(lhs + rhs, Coord(1, 1));

        let lhs = Coord(10, 0);
        let rhs = Coord(1, 0);

        assert_eq!(lhs + rhs, Coord(10, 0));
    }

    #[test]
    fn should_order() {
        set_max_width(10);
        set_max_height(10);
        let mut coords = vec![Coord(0, 9), Coord(9, 1), Coord(0, 0), Coord(10, 10)];
        coords.sort();

        assert_eq!(coords, vec![Coord(0, 0), Coord(9, 1), Coord(0, 9), Coord(10, 10)]);
    }
}