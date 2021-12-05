use std::cmp::Ordering;
use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<((i64, i64), (i64, i64))>;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" -> ");
            let mut start = split.next().unwrap().split(',');
            let mut stop = split.next().unwrap().split(',');
            (
                (
                    start.next().unwrap().parse().unwrap(),
                    start.next().unwrap().parse().unwrap(),
                ),
                (
                    stop.next().unwrap().parse().unwrap(),
                    stop.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Parsed) -> usize {
    let mut graph: HashMap<(i64, i64), u64> = HashMap::new();
    input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .for_each(|((x1, y1), (x2, y2))| {
            let mut x = *x1;
            let mut y = *y1;
            loop {
                *graph.entry((x, y)).or_insert(0) += 1;

                if y == *y2 && x == *x2 {
                    break;
                }
                match y.cmp(y2) {
                    Ordering::Greater => y -= 1,
                    Ordering::Less => y += 1,
                    Ordering::Equal => {}
                }
                match x.cmp(x2) {
                    Ordering::Greater => x -= 1,
                    Ordering::Less => x += 1,
                    Ordering::Equal => {}
                }
            }
        });

    graph.values().filter(|v| **v > 1).count()
}

#[aoc(day5, part2)]
fn part2(input: &Parsed) -> usize {
    let mut graph: HashMap<(i64, i64), u64> = HashMap::new();

    input
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2 || (y2 - y1).abs() == (x2 - x1).abs())
        .for_each(|((x1, y1), (x2, y2))| {
            let mut x = *x1;
            let mut y = *y1;
            loop {
                *graph.entry((x, y)).or_insert(0) += 1;

                if y == *y2 && x == *x2 {
                    break;
                }
                match y.cmp(y2) {
                    Ordering::Greater => y -= 1,
                    Ordering::Less => y += 1,
                    Ordering::Equal => {}
                }
                match x.cmp(x2) {
                    Ordering::Greater => x -= 1,
                    Ordering::Less => x += 1,
                    Ordering::Equal => {}
                }
            }
        });

    graph.values().filter(|v| **v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(part1(&parse_input(input)), 5);
    }

    #[test]
    fn sample2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(part2(&parse_input(input)), 12);
    }
}
