use std::fs;

mod q_1;
mod q_2;
mod q_3;
mod q_4;
mod q_5;
mod q_6;
mod q_7;

fn main() {
    let q_input = fs::read_to_string("inputs/q7.txt")
        .expect("Could not read input!")
        .replace("\r", "");
    q_7::run(&q_input);
}
