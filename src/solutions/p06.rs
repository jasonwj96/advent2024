/*
--- Day 6: Guard Gallivant ---

The Historians use their fancy device again, this time to whisk you all away to the North Pole
prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518
while The Historians search for the Chief. Unfortunately, a single guard is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?

You start by making a map (your puzzle input) of the situation. For example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

The map shows the current position of the guard with ^ (to indicate the guard is currently facing up
from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

    If there is something directly in front of you, turn right 90 degrees.
    Otherwise, take a step forward.

Following the above protocol, the guard moves up several times until she reaches an obstacle
(in this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Because there is now an obstacle in front of the guard, she turns right before continuing
straight in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Reaching another obstacle (a spool of several very long polymers),
she turns right again and continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...

This process continues for a while, but the guard eventually leaves the mapped area
(after walking past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..

By predicting the guard's route, you can determine which specific positions in the
lab will be in the patrol path. Including the guard's starting position, the positions
visited by the guard before leaving the area are marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..

In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard visit
before leaving the mapped area?

*/

use fxhash::FxHashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::{Down, Left, Right, Up};

impl Direction {
    fn turn_right_90_degrees(self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<Direction> for usize {
    fn from(d: Direction) -> Self {
        match d {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
    guard_starting_position: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        let guard_starting_position = values.iter().position(|&c| c == '^').unwrap();
        Self {
            values,
            rows,
            cols,
            guard_starting_position,
        }
    }

    fn allowed(&self, pos: usize, direction: Direction) -> bool {
        !match direction {
            Up => pos < self.cols,
            Right => pos % self.cols == self.cols - 1,
            Down => pos / self.cols == self.rows - 1,
            Left => pos % self.cols == 0,
        }
    }

    // Assumes validity of the move has been checked before with `allowed`.
    fn next_pos(&self, pos: usize, direction: Direction) -> usize {
        match direction {
            Up => pos - self.cols,
            Right => pos + 1,
            Down => pos + self.cols,
            Left => pos - 1,
        }
    }
}

// Part 1. Could be merged with part 2, but readability may be worse.
fn visited_positions_count(map: &Grid) -> usize {
    // A grid of the same size as the map to mark the visited positions.
    let mut visited = vec![false; map.values.len()];

    let mut guard_pos = map.guard_starting_position;
    visited[guard_pos] = true;

    let mut direction = Direction::Up;
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        match map.values.get(next_pos) {
            Some('#') => {
                direction = direction.turn_right_90_degrees();
            }
            Some('.' | '^') => {
                guard_pos = next_pos;
                visited[guard_pos] = true;
            }
            _ => panic!("Invalid map element"),
        }
    }
    // debug::simple(map, &visited);

    visited.iter().filter(|&&v| v).count()
}

// Walks the map, starting from guard_pos / direction.
// Returns true if we reach a loop, false if we get out.
fn walk_until_loop(
    map: &Grid,
    extra_obstacle_pos: usize,
    mut guard_pos: usize,
    mut direction: Direction,
    mut visited: Vec<[bool; 4]>,
) -> bool {
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        if next_pos == extra_obstacle_pos || matches!(map.values.get(next_pos), Some('#')) {
            direction = direction.turn_right_90_degrees();
            visited[guard_pos][usize::from(direction)] = true; // only matters for debug printing.
        } else {
            assert!(visited[guard_pos][usize::from(direction)]);
            if visited[next_pos][usize::from(direction)] {
                // println!("Loop (at {next_pos}):");
                // debug::print(map, extra_obstacle_pos, &visited, &[guard_pos], true);

                return true;
            }
            guard_pos = next_pos;
            visited[guard_pos][usize::from(direction)] = true;
        }
    }
    false
}

// Part 2
fn obstruction_positions_count(map: &Grid) -> usize {
    // A loop happens when we reach a previously visited place with the same direction.
    // So as we walk through the map, on each step we try to place an obstruction and check if we reach a loop.

    // We cannot simply count obstructions as different path could loop with obstructions in the same spot.
    let mut obstructions: FxHashSet<usize> = FxHashSet::default();

    // Visited positions with directions.
    let mut visited: Vec<[bool; 4]> = vec![[false; 4]; map.values.len()];

    let mut guard_pos = map.guard_starting_position;
    visited[guard_pos][usize::from(Up)] = true;

    let mut direction = Up;
    while map.allowed(guard_pos, direction) {
        let next_pos = map.next_pos(guard_pos, direction);
        match map.values.get(next_pos) {
            Some('#') => {
                direction = direction.turn_right_90_degrees();
                visited[guard_pos][usize::from(direction)] = true; // only matters for debug printing.
            }
            Some('.' | '^') => {
                // If next position is free, test if putting an obstacle would result in a loop.
                // - We don't need to check it if we have already found a working obstruction there before.
                // - The new obstruction can't be placed at the guard's starting position.
                // - We cannot put an obstacle on an already visited position.
                if !obstructions.contains(&next_pos)
                    && next_pos != map.guard_starting_position
                    && !visited[next_pos].iter().any(|&v| v)
                    && walk_until_loop(map, next_pos, guard_pos, direction, visited.clone())
                {
                    obstructions.insert(next_pos);
                }

                guard_pos = next_pos;
                visited[guard_pos][usize::from(direction)] = true;
            }
            _ => panic!("Invalid map element"),
        }
    }

    obstructions.len()
}


pub fn part1() -> usize {
    let map = Grid::build(include_str!("../inputs/06.txt"));
    visited_positions_count(&map)
}

pub fn part2() -> usize {
    let map = Grid::build(include_str!("../inputs/06.txt"));
    obstruction_positions_count(&map)
}