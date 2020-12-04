#![feature(test)]
extern crate hashbrown;
extern crate packed_simd;
extern crate test;
extern crate utils;

#[allow(dead_code)]
mod day_4;
// mod day_3;
// mod day_2;
// mod day_1;

fn main() {
    day_4::run();
}
