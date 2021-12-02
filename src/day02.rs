use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

enum Moves {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Moves> {
    input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.splitn(2, ' ').collect();
            let n = split[1].parse().unwrap();
            match split[0] {
                "forward" => Moves::Forward(n),
                "down" => Moves::Down(n),
                "up" => Moves::Up(n),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Moves]) -> i32 {
    let (hp, d) = input.iter().fold((0, 0), |(hp, d), m| match m {
        Moves::Forward(n) => (hp + n, d),
        Moves::Up(n) => (hp, d - n),
        Moves::Down(n) => (hp, d + n),
    });
    hp * d
}

#[aoc(day2, part2)]
fn part2(input: &[Moves]) -> i32 {
    let (hp, d, _) = input.iter().fold((0, 0, 0), |(hp, d, a), m| match m {
        Moves::Forward(n) => (hp + n, d + a * n, a),
        Moves::Up(n) => (hp, d, a - n),
        Moves::Down(n) => (hp, d, a + n),
    });
    hp * d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part1(&parse_input(input)), 150);
    }
    #[test]
    fn sample2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(part2(&parse_input(input)), 900);
    }
}
