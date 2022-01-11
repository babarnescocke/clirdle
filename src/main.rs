use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("{}", random_line_from_file(&"../5_letter_words.txt".to_string()));
}

fn random_line_from_file(input: &String) -> String {
let f = File::open(input)
        .unwrap_or_else(|e| panic!("(;_;) file not found: {}: {}", input, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}