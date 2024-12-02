use counter::Counter;
use std::{collections::HashMap, fs, hash::Hash};

fn main() {
    let mut left_side: Vec<i64> = Vec::new();
    let mut right_side: Vec<i64> = Vec::new();

    for line in fs::read_to_string("./01/src/input.txt").unwrap().lines() {
        let mut line_vals = line.split_whitespace();

        left_side.push(
            line_vals
                .next()
                .expect("Left side no value")
                .parse()
                .expect("Left side not an i64"),
        );

        right_side.push(
            line_vals
                .next()
                .expect("Right side no value")
                .parse()
                .expect("Right side not an i64"),
        );
    }

    left_side.sort();
    right_side.sort();

    let mut i = 0;
    let mut total = 0;
    while i < left_side.len() {
        let diff = (right_side[i] - left_side[i]).abs();
        let value = left_side[i];

        i += 1;
        total += diff;
    }

    let right_counts = right_side.iter().collect::<Counter<_>>();
    let mut total_products: Vec<i64> = Vec::new();
    for n in 0..left_side.len() {
        total_products.push(left_side[n] * right_counts[&left_side[n]] as i64);
    }
    println!("(Total)={}", total);
    println!("(Count Total)={:?}", total_products.iter().sum::<i64>());
}
