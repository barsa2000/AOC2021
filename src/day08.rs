use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<(Vec<u8>, Vec<u8>)>;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(" | ");
            let patterns = split.next().unwrap();
            let output = split.next().unwrap();
            (
                patterns
                    .split(' ')
                    .map(|p| {
                        p.chars()
                            .fold(0, |tot, c| tot | 1 << (c as u32 - 'a' as u32))
                    })
                    .collect(),
                output
                    .split(' ')
                    .map(|p| {
                        p.chars()
                            .fold(0, |tot, c| tot | 1 << (c as u32 - 'a' as u32))
                    })
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &Parsed) -> u64 {
    input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|o| matches!(o.count_ones(), 2 | 4 | 3 | 7))
                .count() as u64
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &Parsed) -> u64 {
    input
        .iter()
        .map(|(pattern, output)| {
            let mut digits: HashMap<u8, &u8> = HashMap::new();
            let mut segments: HashMap<u8, u8> = HashMap::new();
            pattern.iter().for_each(|o| match o.count_ones() {
                2 => {
                    digits.insert(1, o);
                }
                4 => {
                    digits.insert(4, o);
                }
                3 => {
                    digits.insert(7, o);
                }
                7 => {
                    digits.insert(8, o);
                }
                _ => {}
            });

            segments.insert(0, **digits.get(&7).unwrap() ^ **digits.get(&1).unwrap());
            digits.insert(
                3,
                pattern
                    .iter()
                    .find(|p| {
                        p.count_ones() == 5
                            && (**p & **digits.get(&7).unwrap() ^ **digits.get(&7).unwrap()) == 0
                    })
                    .unwrap(),
            );
            digits.insert(
                6,
                pattern
                    .iter()
                    .find(|p| {
                        p.count_ones() == 6
                            && (**p & **digits.get(&1).unwrap() ^ **digits.get(&1).unwrap())
                                .count_ones()
                                == 1
                    })
                    .unwrap(),
            );
            segments.insert(
                1,
                **digits.get(&1).unwrap() ^ **digits.get(&6).unwrap() & **digits.get(&1).unwrap(),
            );

            segments.insert(2, **digits.get(&1).unwrap() ^ **digits.get(&7).unwrap());
            digits.insert(
                5,
                pattern
                    .iter()
                    .find(|p| {
                        p.count_ones() == 5
                            && **p & *segments.get(&1).unwrap() == 0
                            && (**p & **digits.get(&6).unwrap()).count_ones() == 5
                    })
                    .unwrap(),
            );
            segments.insert(4, **digits.get(&5).unwrap() ^ **digits.get(&6).unwrap());
            digits.insert(
                9,
                pattern
                    .iter()
                    .find(|p| p.count_ones() == 6 && **p & *segments.get(&4).unwrap() == 0)
                    .unwrap(),
            );
            digits.insert(
                0,
                pattern
                    .iter()
                    .find(|p| {
                        p.count_ones() == 6
                            && **p & *segments.get(&4).unwrap() != 0
                            && **p & *segments.get(&1).unwrap() != 0
                    })
                    .unwrap(),
            );
            digits.insert(
                2,
                pattern
                    .iter()
                    .find(|p| p.count_ones() == 5 && **p & *segments.get(&4).unwrap() != 0)
                    .unwrap(),
            );

            output
                .iter()
                .map(|out_digit| *digits.iter().find(|(_, d)| ***d == *out_digit).unwrap().0)
                .fold(0_u64, |tot, d| tot * 10 + d as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(part1(&parse_input(input)), 26);
    }
    #[test]
    fn sample2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(part2(&parse_input(input)), 61229);
    }
}
