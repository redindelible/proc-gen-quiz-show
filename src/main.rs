mod arithmetic;
mod question;
mod calculus;
mod utils;

use std::io;
use rand::prelude::*;

use crate::arithmetic::ArithmeticProblem;
use crate::calculus::RelatedRates;
use crate::question::{Question, CheckAnswer, GenerateQuestion};


fn main() {
    let generators: &[Box<dyn GenerateQuestion>] = &[
        Box::new(ArithmeticProblem { terms: 4..=6 }),
        Box::new(RelatedRates)
    ];

    let mut rng = thread_rng();
    loop {
        let q: Question = generators.choose(&mut rng).unwrap().generate(&mut rng);

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
