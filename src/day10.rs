use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<String>;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Parsed) -> u128 {
    input.iter().map(count_points_if_corrupted).sum()
}

#[aoc(day10, part2)]
fn part2(input: &Parsed) -> u128 {
    let mut points: Vec<u128> = input
        .iter()
        .filter(|l| count_points_if_corrupted(l) == 0)
        .map(|l| {
            let mut queue = vec![];
            for c in l.chars() {
                match c {
                    '(' => queue.push(0),
                    '[' => queue.push(1),
                    '{' => queue.push(2),
                    '<' => queue.push(3),
                    ')' => {
                        queue.pop();
                    }
                    ']' => {
                        queue.pop();
                    }
                    '}' => {
                        queue.pop();
                    }
                    '>' => {
                        queue.pop();
                    }
                    _ => unreachable!(),
                };
            }
            queue.iter().rev().fold(0, |a, q| a * 5 + q + 1)
        })
        .collect();
    points.sort_unstable();
    points[points.len() / 2]
}

fn count_points_if_corrupted(l: &String) -> u128 {
    let mut queue = vec![];
    for c in l.chars() {
        match c {
            '(' => queue.push(0),
            '[' => queue.push(1),
            '{' => queue.push(2),
            '<' => queue.push(3),
            ')' => {
                if let Some(q) = queue.pop() {
                    if q != 0 {
                        return 3;
                    }
                }
            }
            ']' => {
                if let Some(q) = queue.pop() {
                    if q != 1 {
                        return 57;
                    }
                }
            }
            '}' => {
                if let Some(q) = queue.pop() {
                    if q != 2 {
                        return 1197;
                    }
                }
            }
            '>' => {
                if let Some(q) = queue.pop() {
                    if q != 3 {
                        return 25137;
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 26397);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 288957);
    }
}
