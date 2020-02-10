use crate::core::coordinate::Coord;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;

pub(crate) const EAST: Coord = Coord(1, 0);
pub(crate) const WEST: Coord = Coord(-1, 0);
pub(crate) const NORTH: Coord = Coord(0, -1);
pub(crate) const SOUTH: Coord = Coord(0, 1);
pub(crate) const SOUTH_EAST: Coord = Coord(1, 1);
pub(crate) const SOUTH_WEST: Coord = Coord(-1, 1);
pub(crate) const NORTH_EAST: Coord = Coord(1, -1);
pub(crate) const NORTH_WEST: Coord = Coord(-1, -1);

pub(crate) fn max_width() -> i32 {
    unsafe { WIDTH }
}

pub(crate) fn set_max_width(width: i32) {
    unsafe { WIDTH = width }
}

pub(crate) fn max_height() -> i32 {
    unsafe { HEIGHT }
}

pub(crate) fn size() -> i32 {
    unsafe { HEIGHT * HEIGHT }
}

pub(crate) fn set_max_height(height: i32) {
    unsafe { HEIGHT = height }
}
