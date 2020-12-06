#![feature(test)]
#![feature(iterator_fold_self)]
extern crate hashbrown;
extern crate packed_simd;
extern crate test;
extern crate utils;

#[allow(dead_code)]
mod day_6;
// mod day_5;
// mod day_4;
// mod day_3;
// mod day_2;
// mod day_1;

fn main() {
    day_6::run();
}
