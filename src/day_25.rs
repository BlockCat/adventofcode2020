struct Cont {
    key_loop: Option<usize>,
    door_loop: Option<usize>,
}
// #[test]
pub fn run() {

    // println!("{}", exercise_1(5764801, 17807724));
    println!("{}", exercise_1(19241437, 17346587));
    // println!("{}", exercise_2(&input));
}

fn exercise_1(public_key: usize, public_door: usize) -> usize {
    let mut key_val = 1;
    let mut public_key_val = 1;
    let mut public_door_val = 1;

    let mut key_loops = None;
    let mut door_loops = None;

    for i in 1.. {
        key_val = transform(key_val, 7);
        public_key_val = transform(public_key_val, public_key);
        public_door_val = transform(public_door_val, public_door);

        if key_val == public_key {
            key_loops = Some(i);
            println!("key found: {} at loop: {} with door at: {}", key_val, i, public_door_val );
            return public_door_val;
        }
        if key_val == public_door {
            door_loops = Some(i);
            println!("door found: {} at loop: {} with key at: {}", key_val, i, public_key_val );
            return public_key_val;
        }
    }
    unreachable!()
}

fn transform(v: usize, subject: usize) -> usize {
    (v * subject) % 20201227
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test::Bencher;

//     #[test]
//     fn d18ex1() {
//         let input = read_input(include_str!("input/day25test.txt"));
//         assert_eq!(2, exercise_1(&input));
//         // assert_eq!(71, exercise_1(&input))
//     }

//     #[test]
//     fn d18ex2() {
//         let input = read_input(include_str!("input/day25test.txt"));
//         assert_eq!(12, exercise_1(&input));
//     }

//     #[bench]
//     fn d18_bench_ex1(b: &mut Bencher) {
//         let input = read_input(include_str!("input/day25.txt"));
//         b.iter(|| exercise_1(&input));
//     }

//     #[bench]
//     fn d18_bench_ex2(b: &mut Bencher) {
//         let input = read_input(include_str!("input/day25.txt"));
//         b.iter(|| exercise_1(&input));
//     }
// }
