use crate::util::grid::*;
use std::array::from_fn;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    0
}

pub fn part2(grid: &Grid<u8>) -> usize {
    0
}

/// There are 8 neighbours to check, giving 2⁸ possibilities. Precompute the number of corners
/// once into a lookup table to speed things up.
fn sides_lut() -> [usize; 256] {
    from_fn(|neighbours| {
        let [up_left, up, up_right, left, right, down_left, down, down_right] =
            from_fn(|i| neighbours & (1 << i) != 0);
        let mut sides = 0;

        if !(up || left) || (up && left && !up_left) {
            sides += 1;
        }
        if !(up || right) || (up && right && !up_right) {
            sides += 1;
        }
        if !(down || left) || (down && left && !down_left) {
            sides += 1;
        }
        if !(down || right) || (down && right && !down_right) {
            sides += 1;
        }

        sides
    })
}
