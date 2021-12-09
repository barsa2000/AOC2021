use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{self, Array2, Axis};

type Parsed = Array2<u32>;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Parsed {
    let v: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    Array2::from_shape_vec(
        (v.len(), v.first().unwrap().len()),
        v.iter().flatten().copied().collect(),
    )
    .unwrap()
}

#[aoc(day9, part1)]
fn part1(input: &Parsed) -> u32 {
    let mut sum = 0;
    let rows = input.len_of(Axis(0));
    let cols = input.len_of(Axis(1));
    for r in 0..rows {
        for c in 0..cols {
            let curr = input.get((r, c)).unwrap();
            if (r == 0 || curr < input.get((r - 1, c)).unwrap())
                && (r == rows - 1 || curr < input.get((r + 1, c)).unwrap())
                && (c == 0 || curr < input.get((r, c - 1)).unwrap())
                && (c == cols - 1 || curr < input.get((r, c + 1)).unwrap())
            {
                sum += curr + 1;
            }
        }
    }
    sum
}

fn flood(arr: &mut Array2<u32>, pos: (i32, i32)) -> u64 {
    if pos.0 >= 0
        && pos.0 < arr.len_of(Axis(0)) as i32
        && pos.1 >= 0
        && pos.1 < arr.len_of(Axis(1)) as i32
        && *arr.get((pos.0 as usize, pos.1 as usize)).unwrap() != 9
    {
        *arr.get_mut((pos.0 as usize, pos.1 as usize)).unwrap() = 9;
        1 + flood(arr, (pos.0 - 1, pos.1))
            + flood(arr, (pos.0 + 1, pos.1))
            + flood(arr, (pos.0, pos.1 - 1))
            + flood(arr, (pos.0, pos.1 + 1))
    } else {
        0
    }
}

#[aoc(day9, part2)]
fn part2(input: &Parsed) -> u64 {
    let mut basins = vec![];
    let rows = input.len_of(Axis(0));
    let cols = input.len_of(Axis(1));
    for r in 0..rows {
        for c in 0..cols {
            let curr = input.get((r, c)).unwrap();
            if (r == 0 || curr < input.get((r - 1, c)).unwrap())
                && (r == rows - 1 || curr < input.get((r + 1, c)).unwrap())
                && (c == 0 || curr < input.get((r, c - 1)).unwrap())
                && (c == cols - 1 || curr < input.get((r, c + 1)).unwrap())
            {
                basins.push(flood(&mut input.to_owned(), (r as i32, c as i32)));
            }
        }
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));
    basins[0] * basins[1] * basins[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(part1(&parse_input(input)), 15);
    }
    #[test]
    fn sample2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(part2(&parse_input(input)), 1134);
    }
}
