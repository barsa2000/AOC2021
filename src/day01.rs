use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(3))
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(&input), 7);
    }
    #[test]
    fn sample2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part2(&input), 5);
    }
}
