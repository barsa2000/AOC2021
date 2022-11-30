use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<Vec<u128>>;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u128).collect())
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &Parsed) -> u128 {
    let mut map = input.clone();
    let mut tot = 0;
    for _ in 0..100 {
        tot += step(&mut map);
    }
    tot
}

#[aoc(day11, part2)]
fn part2(input: &Parsed) -> u128 {
    let mut map = input.clone();
    let mut i = 0;
    loop {
        if map.iter().flatten().all(|m| *m == 0) {
            break;
        }
        step(&mut map);
        i += 1;
    }
    i
}

fn step(map: &mut Parsed) -> u128 {
    map.iter_mut().flatten().for_each(|m| *m += 1);
    let mut flashed = vec![vec![false; map[0].len()]; map.len()];

    loop {
        if !calc_overflows(map, &mut flashed) {
            break;
        }
    }
    map.iter_mut().flatten().fold(0, |ctr, m| {
        if *m > 9 {
            *m = 0;
            ctr + 1
        } else {
            ctr
        }
    })
}

fn calc_overflows(map: &mut Parsed, flashed: &mut Vec<Vec<bool>>) -> bool {
    let mut changed = false;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !flashed[y][x] && map[y][x] >= 10 {
                changed = true;
                flashed[y][x] = true;
                if x == 0 {
                    if y == 0 {
                        map[y][x + 1] += 1;
                        map[y + 1][x + 1] += 1;
                        map[y + 1][x] += 1;
                    } else if y == map.len() - 1 {
                        map[y][x + 1] += 1;
                        map[y - 1][x + 1] += 1;
                        map[y - 1][x] += 1;
                    } else {
                        map[y][x + 1] += 1;
                        map[y + 1][x + 1] += 1;
                        map[y - 1][x + 1] += 1;
                        map[y + 1][x] += 1;
                        map[y - 1][x] += 1;
                    }
                } else if x == map[y].len() - 1 {
                    if y == 0 {
                        map[y][x - 1] += 1;
                        map[y + 1][x - 1] += 1;
                        map[y + 1][x] += 1;
                    } else if y == map.len() - 1 {
                        map[y][x - 1] += 1;
                        map[y - 1][x - 1] += 1;
                        map[y - 1][x] += 1;
                    } else {
                        map[y][x - 1] += 1;
                        map[y + 1][x - 1] += 1;
                        map[y - 1][x - 1] += 1;
                        map[y + 1][x] += 1;
                        map[y - 1][x] += 1;
                    }
                } else {
                    if y == 0 {
                        map[y][x + 1] += 1;
                        map[y][x - 1] += 1;
                        map[y + 1][x - 1] += 1;
                        map[y + 1][x] += 1;
                        map[y + 1][x + 1] += 1;
                    } else if y == map.len() - 1 {
                        map[y][x + 1] += 1;
                        map[y][x - 1] += 1;
                        map[y - 1][x - 1] += 1;
                        map[y - 1][x] += 1;
                        map[y - 1][x + 1] += 1;
                    } else {
                        map[y - 1][x - 1] += 1;
                        map[y - 1][x] += 1;
                        map[y - 1][x + 1] += 1;
                        map[y][x - 1] += 1;
                        map[y][x + 1] += 1;
                        map[y + 1][x - 1] += 1;
                        map[y + 1][x] += 1;
                        map[y + 1][x + 1] += 1;
                    }
                }
            }
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 1656);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 195);
    }
}
