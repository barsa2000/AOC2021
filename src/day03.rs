use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1, flipped)]
#[aoc_generator(day3, part2, flipped)]
fn parse_input_flipped(input: &str) -> (Vec<Vec<bool>>, usize, usize) {
    let h = input.lines().next().unwrap().len();
    let w = input.lines().count();
    let mut out = vec![vec![]; h];

    input.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(j, c)| {
            out.get_mut(j).unwrap().push(c.to_digit(10).unwrap() != 0);
        });
    });

    (out, w, h)
}

#[aoc(day3, part1, flipped)]
fn part1_flipped(input: &(Vec<Vec<bool>>, usize, usize)) -> usize {
    let w = input.1;
    //let h = input.2;
    let input = &input.0;
    let (gamma, epsilon) = input
        .iter()
        .map(|v| v.iter().filter(|b| **b).count())
        .fold((0, 0), |(g, e), n| {
            (g << 1 | (n * 2 < w) as usize, e << 1 | (n * 2 > w) as usize)
        });

    gamma * epsilon
}

#[aoc(day3, part2, flipped)]
fn part2_flipped(input: &(Vec<Vec<bool>>, usize, usize)) -> usize {
    let w = input.1;
    let h = input.2;
    let input = &input.0;

    //(oxy_mask, co2_mask)
    let mut mask = vec![(true, true); w];

    for i in 0..h {
        let (count_oxy, count_co2) = mask.iter().fold((0, 0), |(b_acc, c_acc), (bb, cb)| {
            (b_acc + *bb as usize, c_acc + *cb as usize)
        });
        if count_oxy == 1 && count_co2 == 1 {
            break;
        }

        let (ones_oxy, ones_co2) = input.get(i).unwrap().iter().zip(mask.iter()).fold(
            (0, 0),
            |(acc_oxy, acc_co2), (v, (b_oxy, b_co2))| {
                (
                    acc_oxy + (*b_oxy && *v) as usize,
                    acc_co2 + (*b_co2 && *v) as usize,
                )
            },
        );

        input
            .get(i)
            .unwrap()
            .iter()
            .zip(mask.iter_mut())
            .filter(|(_, (m_oxy, m_co2))| *m_oxy || *m_co2)
            .for_each(|(b, (m_oxy, m_co2))| {
                if count_oxy != 1 && *m_oxy {
                    if ones_oxy * 2 != count_oxy {
                        *m_oxy = *b == (ones_oxy * 2 > count_oxy);
                    } else {
                        *m_oxy = *b;
                    }
                }
                if count_co2 != 1 && *m_co2 {
                    *m_co2 =
                        ((ones_co2 * 2 == count_co2) && !*b) || (*b == (ones_co2 * 2 < count_co2));
                }
            });
    }

    let oxy_pos = mask.iter().position(|(m, _)| *m).unwrap();
    let co2_pos = mask.iter().position(|(_, m)| *m).unwrap();
    let (oxy, co2) = input.iter().fold((0, 0), |(acc_o, acc_c), b| {
        (
            acc_o << 1 | *b.get(oxy_pos).unwrap() as usize,
            acc_c << 1 | *b.get(co2_pos).unwrap() as usize,
        )
    });
    oxy * co2
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() != 0).collect())
        .collect()
}

fn count_ones(input: &Vec<Vec<bool>>) -> Vec<usize> {
    let len = input.get(0).unwrap().len();
    input.iter().fold(vec![0usize; len], |mut out, l| {
        l.iter()
            .enumerate()
            .for_each(|(i, &c)| out[i] += c as usize);
        out
    })
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Vec<bool>>) -> usize {
    let count = input.len();
    let (gamma, epsilon) = count_ones(input).iter().fold((0, 0), |(g, e), &n| {
        (
            g << 1 | (n * 2 < count) as usize,
            e << 1 | (n * 2 > count) as usize,
        )
    });

    gamma * epsilon
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Vec<bool>>) -> usize {
    let count = input.len();
    let len = input.get(0).unwrap().len();
    let ones = count_ones(input);
    // println!("{:?}", input);
    // println!("{:?}", ones);

    let mut oxy_vec = input.to_owned();
    let mut oxy_count = count;
    let mut oxy_ones = ones.clone();
    // let mut co2_filtered = input.iter();
    let mut co2_vec = input.to_owned();
    let mut co2_count = count;
    let mut co2_ones = ones;

    for i in 0..len {
        if oxy_count > 1 {
            oxy_vec = oxy_vec
                .iter()
                .filter(|v| {
                    let b = v.get(i).unwrap();
                    let b_ones = oxy_ones.get(i).unwrap();
                    if *b_ones * 2 == oxy_count {
                        *b
                    } else {
                        *b == (*b_ones * 2 > oxy_count)
                    }
                })
                .cloned()
                .collect();

            oxy_ones = count_ones(&oxy_vec);
            oxy_count = oxy_vec.len();

            // println!("{:?}", oxy_ones);
            // println!("{}: {:?}", oxy_count, oxy_vec);
        }
        if co2_count > 1 {
            co2_vec = co2_vec
                .iter()
                .filter(|v| {
                    let b = v.get(i).unwrap();
                    let b_ones = co2_ones.get(i).unwrap();
                    let least_common = *b_ones * 2 < co2_count;
                    //*b == least_common
                    ((*b_ones * 2 == co2_count) && !*b) || (*b == least_common)
                })
                .cloned()
                .collect();

            co2_ones = count_ones(&co2_vec);
            co2_count = co2_vec.len();
            //println!("{:?}", co2_ones);
            //println!("{}: {:?}", co2_count, co2_vec);
        }
    }

    // println!("oxy out: {:?}", oxy_vec);
    // println!("co2 out: {:?}", co2_vec);

    let (oxy, co2) = oxy_vec
        .get(0)
        .unwrap()
        .iter()
        .zip(co2_vec.get(0).unwrap())
        .fold((0, 0), |(o_acc, c_acc), (&o, &c)| {
            (o_acc << 1 | o as usize, c_acc << 1 | c as usize)
        });
    oxy * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part1(&parse_input(input)), 198);
    }

    #[test]
    fn sample2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part2(&parse_input(input)), 230);
    }

    #[test]
    fn sample1_flipped() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part1_flipped(&parse_input_flipped(input)), 198);
    }

    #[test]
    fn sample2_flipped() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(part2_flipped(&parse_input_flipped(input)), 230);
    }
}
