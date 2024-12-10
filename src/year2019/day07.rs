//! # Amplification Circuit
//!
//! Brute force solution for both parts using the utility [`permutations`] method to test each of
//! the possible 5! or 120 permutations of the phase settings.
//!
//! [`permutations`]: crate::util::slice
use super::intcode::*;
use crate::util::parse::*;
use crate::util::slice::*;
use std::array::from_fn;

pub fn parse(input: &str) -> Vec<i64> {
    input.iter_signed::<i64>().collect()
}

pub fn part1(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computer = Computer::new(input);

    let sequence = |slice: &[i64]| {
        let mut signal = 0;

        // Send exactly 2 inputs and receive exactly 1 output per amplifier.
        for &phase in slice {
            computer.reset();
            computer.input(phase);
            computer.input(signal);

            match computer.run() {
                State::Output(next) => signal = next,
                _ => unreachable!(),
            }
        }

        result = result.max(signal);
    };

    [0, 1, 2, 3, 4].permutations(sequence);
    result
}

pub fn part2(input: &[i64]) -> i64 {
    let mut result = 0;
    let mut computers: [Computer; 5] = from_fn(|_| Computer::new(input));

    let feedback = |slice: &[i64]| {
        // Reset state.
        computers.iter_mut().for_each(Computer::reset);

        // Send each initial phase setting exactly once.
        for (computer, &phase) in computers.iter_mut().zip(slice.iter()) {
            computer.input(phase);
        }

        // Chain amplifier inputs and ouputs in a loop until all threads finish.
        let mut signal = 0;

        'outer: loop {
            for computer in &mut computers {
                computer.input(signal);

                match computer.run() {
                    State::Output(next) => signal = next,
                    _ => break 'outer,
                }
            }
        }

        result = result.max(signal);
    };

    [5, 6, 7, 8, 9].permutations(feedback);
    result
}
