/*

--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look.
The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that
are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty
stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even
sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and
hang on did you just say the sky ("of course, where do you think snow comes from") when you realize
that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input)
 has been amended by a very young Elf who was apparently just excited to show off her art skills.
 Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a
specific calibration value that the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77.
Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

Your puzzle answer was 54239.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

Or are they?

The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting,
but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists!
Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.

This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
Calculate a total similarity score by adding up each number in the left list after multiplying it by
the number of times that number appears in the right list.

Here are the same example lists again:

3   4
4   3
2   5
1   3
3   9
3   3

For these example lists, here is the process of finding the similarity score:

    The first number in the left list is 3. It appears in the right list three times,
    so the similarity score increases by 3 * 3 = 9.
    The second number in the left list is 4. It appears in the right list once,
    so the similarity score increases by 4 * 1 = 4.
    The third number in the left list is 2. It does not appear in the right list,
    so the similarity score does not increase (2 * 0 = 0).
    The fourth number, 1, also does not appear in the right list.
    The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
    The last number, 3, appears in the right list three times; the similarity score again increases by 9.

So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).

Once again consider your left and right lists. What is their similarity score?

*/
use itertools::Itertools;
use std::convert::TryFrom;

pub fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

pub fn total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .sorted_unstable()
        .zip(right_list.iter().sorted_unstable())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

pub fn similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|left_element| {
            let count = right_list
                .iter()
                .filter(|right_element| *right_element == left_element)
                .count();
            left_element * u32::try_from(count).unwrap()
        })
        .sum()
}

pub fn part1() -> u32 {
    let (left_list, right_list) = build(include_str!("../inputs/01.txt"));
    total_distance(&left_list, &right_list)
}

pub fn part2() -> u32 {
    let (left_list, right_list) = build(include_str!("../inputs/01.txt"));
    similarity_score(&left_list, &right_list)
}