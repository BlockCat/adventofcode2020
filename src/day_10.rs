// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day10.txt"));
    println!("{:?}", input);
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn read_input(input: &str) -> Vec<usize> {
    let mut lines = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    lines.push(0);
    lines.sort();
    lines.push(lines.last().unwrap() + 3);
    lines
}

fn exercise_1(slice: &Vec<usize>) -> usize {
    let (one, three) =
        slice
            .iter()
            .zip(slice.iter().skip(1))
            .fold((0, 0), |(one, three), (&a, &b)| {
                if b == a + 1 {
                    (one + 1, three)
                } else if b == a + 3 {
                    (one, three + 1)
                } else {
                    (one, three)
                }
            });

    println!("{} of 1 and {} of 3", one, three + 1);
    (one) * (three)
}

fn exercise_2(slice: &Vec<usize>) -> usize {
    let map = slice
        .iter()
        .enumerate()
        .map(|(i, _)| count_it(slice, i))
        .collect::<Vec<_>>();

    let mut d = vec![0usize; slice.len()];

    for x in &map[0] {
        d[*x] = 1;
    }

    for (index, _) in slice.iter().enumerate() {
        for neighbour_index in &map[index] {
            d[*neighbour_index] += d[index];
        }
    }

    *d.last().unwrap()
}

fn count_it(slice: &Vec<usize>, index: usize) -> Vec<usize> {
    let reachable_by = slice[index + 1..]
        .iter()
        .take_while(|x| **x - slice[index] <= 3)
        .enumerate()
        .map(|x| (x.0 + index + 1))
        .collect::<Vec<_>>();

    reachable_by
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;
    #[test]
    fn d10p1_test() {
        let input = read_input(include_str!("input/day10.txt"));
        assert_eq!(2170, exercise_1(&input));
    }

    #[test]
    fn d10p2_test() {
        let input = read_input(include_str!("input/day10.txt"));
        assert_eq!(24803586664192, exercise_2(&input));
    }

    #[bench]
    fn d10_bench_parse(b: &mut Bencher) {
        b.iter(|| read_input(include_str!("input/day1.txt")));
    }
    #[bench]
    fn d10_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day10.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d10_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day10.txt"));
        b.iter(|| exercise_2(&input));
    }

    // #[bench]
    // fn d1_bench_ex1bigboi(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1bigboi.txt"));
    //     b.iter(|| exercise_1(&input, 99920044));
    // }

    // #[bench]
    // fn d1_bench_ex2bigboi(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1bigboi.txt"));
    //     b.iter(|| exercise_2(&input, 99920044));
    // }

    // #[bench]
    // fn d1_bench_parsebigboi(b: &mut Bencher) {
    //     b.iter(|| read_input(include_str!("input/day1bigboi.txt")));
    // }

    // #[bench]
    // fn d1_bench_ex2(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1.txt"));
    //     b.iter(|| exercise_2(&input, 2020));
    // }

    // #[bench]
    // fn d1_bench_ex2b(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1.txt"));
    //     b.iter(|| exercise_2b(&input, 2020));
    // }
}
