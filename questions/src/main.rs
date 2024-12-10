use std::cmp;
use std::{i64, io};

struct pair {
    first : i64,
    second : i64,
}

type GraphType = Vec<Vec<pair>>;

fn solve() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let mut vals = input.split(" ").map(|val| val.parse::<i64>().unwrap());

    let n = vals.next().unwrap();
    let m = vals.next().unwrap();

    for
}

fn main() {
    solve();
}
