//! # Garden Groups
//!
<<<<<<< HEAD
//! Part one flood fills each region, adding 4 to the perimeter for each plot
//! then subtracting 2 for each neighbour that we've already added.
//!
//! Part two counts corners, as the number of corners equals the number of sides.
//! We remove a corner when another plot is adjacent either up, down, left or right:
//!
//! ```none
//!     .#.    ...
//!     .#.    ##.
//!     ...    ...
//! ```
//!
//! We add back a corner when it's concave, for example where a plot is above, right but
//! not above and to the right:
//!
//! ```none
//!     .#.
//!     .##
//!     ...
//! ```
//!
//! There are 8 neighbours to check, giving 2⁸ possibilities. These are precomputed and cached
//! in a lookup table.
use crate::util::grid::*;
use crate::util::point::*;
use std::array::from_fn;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1(grid: &Grid<u8>) -> i32 {
    let mut result = 0;
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut added = grid.same_size_with(false);
=======
//! Solves both parts simultaneously by flood filling each region.
//!
//! For part one we increment the perimeter for each neighbouring plot belonging to a different
//! region or out of bounds.
//!
//! For part two we count each plot on the edge as either 0, 1 or 2 sides then divide by 2.
//! An edge plot contributes nothing if it has 2 edge neighbours facing the same way,
//! one if has a single neighbour and two if it has no neighbours.
//!
//! For example, considering the right edge:
//!
//! ```none
//!     ...        ...        .#. > 1
//!     .#. > 2    .#. > 1    .#. > 0
//!     ...        .#. > 1    .#. > 1
//! ```
use crate::util::grid::*;
use crate::util::point::*;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let mut todo = Vec::new();
    let mut edge = Vec::new();
    let mut seen = grid.same_size_with(false);

    let mut part_one = 0;
    let mut part_two = 0;
>>>>>>> 13b206d8d54375ef008cc746b000ef375f2351eb

    for y in 0..grid.height {
        for x in 0..grid.width {
            // Skip already filled points.
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

<<<<<<< HEAD
            // Flood fill each region.
=======
            // Flood fill, using area as an index.
>>>>>>> 13b206d8d54375ef008cc746b000ef375f2351eb
            let kind = grid[point];
            let check = |point| grid.contains(point) && grid[point] == kind;

            let mut area = 0;
            let mut perimeter = 0;
<<<<<<< HEAD

            todo.push_back(point);
            seen[point] = true;

            while let Some(point) = todo.pop_front() {
                added[point] = true;
                area += 1;
                perimeter += 4;

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind {
                        if !seen[next] {
                            seen[next] = true;
                            todo.push_back(next);
                        }
                        // Remove both sides from neighbouring plots.
                        if added[next] {
                            perimeter -= 2;
                        }
                    }
                }
            }

            result += area * perimeter;
        }
    }

    result
}

pub fn part2(grid: &Grid<u8>) -> usize {
    // Lookup table that returns number of corners for all combinations of neighbours.
    let lut = sides_lut();

    let mut result = 0;
    let mut todo = VecDeque::new();
    let mut seen = grid.same_size_with(false);
    let mut added = grid.same_size_with(-1);
    let mut region = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point::new(x, y);
            if seen[point] {
                continue;
            }

            let kind = grid[point];
            let id = y * grid.width + x;
=======
>>>>>>> 13b206d8d54375ef008cc746b000ef375f2351eb
            let mut sides = 0;

            todo.push(point);
            seen[point] = true;

<<<<<<< HEAD
            while let Some(point) = todo.pop_front() {
                added[point] = id;
                region.push(point);

                for next in ORTHOGONAL.map(|o| point + o) {
                    if grid.contains(next) && grid[next] == kind && !seen[next] {
                        seen[next] = true;
                        todo.push_back(next);
=======
            while area < todo.len() {
                let point = todo[area];
                area += 1;

                for direction in ORTHOGONAL {
                    let next = point + direction;

                    if check(next) {
                        if !seen[next] {
                            todo.push(next);
                            seen[next] = true;
                        }
                    } else {
                        edge.push((point, direction));
                        perimeter += 1;
>>>>>>> 13b206d8d54375ef008cc746b000ef375f2351eb
                    }
                }
            }

<<<<<<< HEAD
            for &point in &region {
                let index = DIAGONAL.iter().fold(0, |acc, &d| {
                    (acc << 1) | (added.contains(point + d) && added[point + d] == id) as usize
                });
                sides += lut[index];
            }

            result += region.len() * sides;
            region.clear();
=======
            // Sum sides for all plots in the region.
            for &(p, d) in &edge {
                let r = d.clockwise();
                let l = d.counter_clockwise();

                sides += (!check(p + l) || check(p + l + d)) as usize;
                sides += (!check(p + r) || check(p + r + d)) as usize;
            }

            todo.clear();
            edge.clear();

            part_one += area * perimeter;
            part_two += area * (sides / 2);
>>>>>>> 13b206d8d54375ef008cc746b000ef375f2351eb
        }
    }

    (part_one, part_two)
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
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
