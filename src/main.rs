#![feature(test)]
extern crate hashbrown;
extern crate test;
extern crate utils;
extern crate packed_simd;

#[allow(dead_code)]
mod day_2;
// mod day_1;

fn main() {    
    day_2::run();
}