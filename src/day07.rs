use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = (Vec<u64>, u64);

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Parsed {
    let parsed: Vec<u64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    (parsed.to_owned(), *parsed.iter().max().unwrap())
}

#[aoc(day7, part1)]
fn part1(input: &Parsed) -> u64 {
    let mut positions = input
        .0
        .iter()
        .fold(vec![0; input.1 as usize + 1], |mut v, n| {
            *v.get_mut(*n as usize).unwrap() += 1;
            v
        });

    let mut base: u64 = 0;

    let mut len = positions.len().to_owned();
    while len > 1 {
        let (mut left, mut right) = positions.split_at_mut(len / 2);

        let sum_left = left.iter().sum::<u64>();
        let sum_right = right.iter().sum::<u64>();

        if sum_left > sum_right {
            let left_last = left.get_mut(left.len() - 1).unwrap();
            *left_last += sum_right;
            positions.drain(len / 2..);
        } else {
            let right_first = right.get_mut(0).unwrap();
            *right_first += sum_left;
            positions.drain(..len / 2);
            base += len as u64 / 2;
        }
        len = positions.len().to_owned();
    }

    input
        .0
        .iter()
        .map(|n| match n.cmp(&base) {
            std::cmp::Ordering::Less => base - *n,
            std::cmp::Ordering::Greater => *n - base,
            std::cmp::Ordering::Equal => 0,
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Parsed) -> u64 {
    let max = input.1;
    let costs: Vec<Vec<u64>> = input
        .0
        .iter()
        .map(|in_pos| {
            (0..=max)
                .map(|i| {
                    if *in_pos > i {
                        (i..*in_pos).map(|p| *in_pos - p).sum()
                    } else {
                        (*in_pos..=i).map(|p| p - *in_pos).sum()
                    }
                })
                .collect::<Vec<u64>>()
        })
        .collect();

    (0..=max)
        .map(|p| {
            costs
                .iter()
                .map(|c| *c.get(p as usize).unwrap())
                .sum::<u64>()
        })
        .min()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part1(&parse_input(input)), 37);
    }

    #[test]
    fn sample2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(part2(&parse_input(input)), 168);
    }
}
