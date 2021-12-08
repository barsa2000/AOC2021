use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<i64>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Parsed {
    todo!()
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> i32 {
    todo!()
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "";
        assert_eq!(part1(&parse_input(input)), 58);
    }
}
