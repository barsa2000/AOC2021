use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::Array2;

type Parsed = (Vec<u32>, Vec<Array2<(u32, bool)>>);

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Parsed {
    let mut split = input.split("\n\n");
    let nums_str = split.next().unwrap().to_owned();
    let cards_str: Vec<&str> = split.collect();

    let nums: Vec<u32> = nums_str
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let cards: Vec<Array2<(u32, bool)>> = cards_str
        .iter()
        .map(|c_s| {
            Array2::from_shape_vec(
                (5, 5),
                c_s.split_ascii_whitespace()
                    .map(|s| (s.parse().unwrap(), false))
                    .collect(),
            )
            .unwrap()
        })
        .collect();

    (nums, cards)
}

fn check_board(board: &Array2<(u32, bool)>) -> bool {
    board.rows().into_iter().any(|r| r.iter().all(|c| c.1))
        || board.columns().into_iter().any(|r| r.iter().all(|c| c.1))
}

#[aoc(day4, part1)]
fn part1(input: &Parsed) -> u32 {
    let nums = &input.0;
    let mut boards = input.1.to_owned();

    for n in nums {
        boards
            .iter_mut()
            .for_each(|b| b.iter_mut().filter(|c| c.0 == *n).for_each(|c| c.1 = true));
        for b in &boards {
            if check_board(b) {
                return b.iter().filter(|c| !c.1).map(|(n, _)| *n).sum::<u32>() * n;
            }
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
fn part2(input: &Parsed) -> u32 {
    let nums = &input.0;
    let mut boards: Vec<(Array2<(u32, bool)>, i32)> =
        input.1.iter().map(|b| (b.clone(), -1)).collect();

    let mut won = 0;
    for n in nums {
        boards
            .iter_mut()
            .filter(|(_, w)| *w == -1)
            .for_each(|(b, _)| b.iter_mut().filter(|c| c.0 == *n).for_each(|c| c.1 = true));

        boards
            .iter_mut()
            .filter(|(_, w)| *w == -1)
            .for_each(|(b, w)| {
                if check_board(b) {
                    *w = won;
                    won += 1;
                }
            });

        if boards.iter().filter(|(_, w)| *w == -1).count() == 0 {
            return boards
                .iter()
                .find(|(_, w)| *w == won - 1)
                .unwrap()
                .0
                .iter()
                .filter(|c| !c.1)
                .map(|(n, _)| *n)
                .sum::<u32>()
                * n;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(part1(&parse_input(input)), 4512);
    }
    #[test]
    fn sample2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(part2(&parse_input(input)), 1924);
    }
}
