use std::mem::swap;
use rand::prelude::*;
use crate::question::{GenerateQuestion, Question};

const TRIPLES: &[(u32, u32, u32)] = &[
    (3, 4, 5), (6, 8, 10), (5, 12, 13), (9, 12, 15), (8, 15, 17), (12, 16, 20), (15, 20, 25), (7, 24, 25),
    // (6, 24, 26), (20, 21, 29), (18, 24, 30), (18, 24, 30), (16, 30, 34), (21, 28, 35), (12, 35, 37), (15, 36, 39)
];

struct RelatedRates;
impl GenerateQuestion for RelatedRates {
    fn generate(&self, rng: &mut dyn RngCore) -> Question {
        let (mut x, mut y, l) = TRIPLES.choose(rng).copied().unwrap();

        if rng.gen_bool(0.5) {
            swap(&mut x, &mut y);
        }

        let vx_known = rng.gen_bool(0.5);
        let x_known = rng.gen_bool(0.5);
        let vx_positive = rng.gen_bool(0.5);

        let (v, answer) = if vx_known {
            let v = y * rng.gen_range(2..=5);
            let answer = x * v / y;
            (v, answer)
        } else {
            let v = x * rng.gen_range(2..=5);
            let answer = y * v / x;
            (v, answer);
        };

        let mut question = format!("A ladder {l} meters long is leaning against a wall. ");

        match (vx_known, vx_positive) {
            (true, true) => {
                question.push_str(&format!("The bottom of the ladder is sliding away from the wall at a rate of {} meters per second. How fast is the top of the ladder sliding down the wall when", v));
            }
            (false, true) => {
                question.push_str(&format!("The top of the ladder is sliding down the wall at a rate of {} meters per second. How fast is the bottom of the ladder sliding away from the wall when", v));
            }
            (true, false) => {
                question.push_str(&format!("The bottom of the ladder is sliding towards the wall at a rate of {} meters per second. How fast is the top of the ladder sliding up the wall when", v));
            }
            (false, false) => {
                question.push_str(&format!("The top of the ladder is sliding up the wall at a rate of {} meters per second. How fast is the bottom of the ladder sliding towards the wall when", v));
            }
        }

        if x_known {
            question.push_str(&format!("the bottom of the ladder is {} meters from the wall? ", x))
        } else {
            question.push_str(&format!("the top of the ladder is {} meters from the floor? ", x))
        }
        question.push_str("(Leave off the units.)");

        todo!()
    }
}