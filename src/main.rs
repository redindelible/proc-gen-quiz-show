mod arithmetic;
mod question;
mod calculus;

use std::io;
use rand::prelude::*;
use arithmetic::ArithmeticProblem;
use question::{CheckAnswer, GenerateQuestion};


fn main() {
    let mut rng = thread_rng();
    loop {
        let q = ArithmeticProblem.generate(&mut rng);

        println!("Question: {}", &q.text);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match q.check_answer.check(input) {
            Ok(()) => {
                println!("Correct!");
            }
            Err(msg) => {
                println!("{msg}")
            }
        }
    }
}
