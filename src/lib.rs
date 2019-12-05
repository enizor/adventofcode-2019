#![feature(const_generics)]

extern crate aoc_runner;
extern crate rayon;
#[macro_use]
extern crate aoc_runner_derive;

pub mod computer;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

aoc_lib! { year = 2019 }
