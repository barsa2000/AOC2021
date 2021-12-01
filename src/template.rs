use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {

}

#[aoc(day1, part1)]
fn part1(boxes: &[i32]) -> i32 {

}

#[aoc(day1, part2)]
fn part2(boxes: &[i32]) -> i32 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input("").unwrap()[..]), 58);
    }

}
