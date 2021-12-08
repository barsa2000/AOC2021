use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<usize>;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Parsed {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &Parsed) -> usize {
    let mut states = input.iter().fold(vec![0; 9], |mut v, n| {
        *v.get_mut(*n).unwrap() += 1;
        v
    });

    for _ in 0..80 {
        let zero = states.remove(0);

        states.push(zero);
        states[6] += zero;
    }

    states.iter().sum()
}

#[aoc(day6, part2)]
fn part2(input: &Parsed) -> usize {
    let mut states = input.iter().fold(vec![0; 9], |mut v, n| {
        *v.get_mut(*n).unwrap() += 1;
        v
    });

    for _ in 0..256 {
        let zero = states.remove(0);

        states.push(zero);
        states[6] += zero;
    }

    states.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "3,4,3,1,2";
        assert_eq!(part1(&parse_input(input)), 5934);
    }

    #[test]
    fn sample2() {
        let input = "3,4,3,1,2";
        assert_eq!(part2(&parse_input(input)), 26984457539);
    }
}
