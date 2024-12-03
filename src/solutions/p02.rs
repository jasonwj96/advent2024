/*
--- Day 2: Red-Nosed Reports ---

Fortunately, the first location The Historians want to search isn't a long walk from the Chief
Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian,
the engineers there run up to you as soon as they see you. Apparently, they still talk about the time
Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing
some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you,
but they seem to have already divided into groups that are currently searching every corner of the facility.
You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line.
Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed
reactor safety systems can only tolerate levels that are either gradually
increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

    7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.

So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?

Your puzzle answer was 306.

--- Part Two ---

The engineers are surprised by the low number of safe reports until they realize they forgot to
tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a
single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report
would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

    7 6 4 2 1: Safe without removing any level.
    1 2 7 8 9: Unsafe regardless of which level is removed.
    9 7 6 2 1: Unsafe regardless of which level is removed.
    1 3 2 4 5: Safe by removing the second level, 3.
    8 6 4 4 1: Safe by removing the third level, 4.
    1 3 6 7 9: Safe without removing any level.

Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level
from unsafe reports. How many reports are now safe?

Your puzzle answer was 366.
*/

pub fn is_valid(input: &Vec<i32>) -> bool {
    let mut is_safe: bool = false;

    if input.len() > 1 {
        let first: i32 = input[0];
        let second: i32 = input[1];

        // Ascending
        if first < second {
            is_safe = input
                .windows(2)
                .all(|pair| (pair[0] - pair[1]).abs() <= 3 && pair[0] < pair[1]);
        }

        // Descending
        if first > second {
            is_safe = input
                .windows(2)
                .all(|pair| (pair[0] - pair[1]).abs() <= 3 && pair[0] > pair[1]);
        }
    }

    is_safe
}

pub fn analyze(input: &str) -> u32 {
    let mut safe_reports: u32 = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_valid(&numbers) {
            safe_reports += 1;
        };
    }

    safe_reports
}

pub fn analyze_with_damping(input: &str) -> u32 {
    let mut safe_reports: u32 = 0;
    let mut is_safe: bool;

    for line in input.lines() {
        let mut dampener: u32 = 0;

        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for idx in 0..numbers.len() {
            let filtered: Vec<i32> = numbers
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, &n)| n)
                .collect();

            if is_valid(&filtered) {
                dampener += 1;
            }
        }

        is_safe = is_valid(&numbers);

        if is_safe || (!is_safe && dampener > 0){
            safe_reports += 1;
        };
    }

    safe_reports
}


pub fn part1() -> u32 {
    analyze(include_str!("../inputs/02.txt"))
}

pub fn part2() -> u32 {
    analyze_with_damping(include_str!("../inputs/02.txt"))
}