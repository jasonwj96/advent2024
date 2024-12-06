/*
--- Day 4: Ceres Search ---

"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes
the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!
+

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt;
she'd like to know if you could help her with her word search (your puzzle input). She only has to
find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards,
or even overlapping other words. It's a little unusual, though, as you don't merely need to find one
instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear,
where irrelevant characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again,
but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear?

*/

use crate::util::grid::*;
use crate::util::point::*;

pub fn parse(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part1() -> u32 {
    let grid: Grid<u8> = parse(include_str!("../inputs/04.txt"));
    let mut result = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);

            if grid[point] == b'X' {
                for step in DIAGONAL {
                    result += (grid.contains(point + step * 3)
                        && grid[point + step] == b'M'
                        && grid[point + step * 2] == b'A'
                        && grid[point + step * 3] == b'S') as u32;
                }
            }
        }
    }

    result
}

pub fn part2() -> u32 {
    let mut result = 0;
    let grid: Grid<u8> = parse(include_str!("../inputs/04.txt"));

    for x in 1..grid.width - 1 {
        for y in 1..grid.height - 1 {
            let point = Point::new(x, y);

            if grid[point] == b'A' {
                let ul = grid[Point::new(x - 1, y - 1)];
                let ur = grid[Point::new(x + 1, y - 1)];
                let dl = grid[Point::new(x - 1, y + 1)];
                let dr = grid[Point::new(x + 1, y + 1)];

                let horizontal = ul == dl && ur == dr && ul.abs_diff(ur) == 6;
                let vertical = ul == ur && dl == dr && ul.abs_diff(dl) == 6;
                result += (horizontal || vertical) as u32;
            }
        }
    }

    result
}