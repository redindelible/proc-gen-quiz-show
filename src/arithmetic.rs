use rand::prelude::*;
use crate::question::{GenerateQuestion, Question};

pub enum Expr {
    Num,
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

impl Expr {
    const OP_ADD: &'static str = "+";
    const OP_SUB: &'static str = "-";
    const OP_MUL: &'static str = "*";
    const OP_DIV: &'static str = "/";

    const OPS: &'static [&'static str] = &[Self::OP_ADD, Self::OP_SUB, Self::OP_MUL, Self::OP_DIV];

    fn primes_less_than(n: u32) -> Vec<u32> {
        let mut primes = vec![];
        for i in 2..=n {
            let is_prime = primes.iter().all(|prime| i % prime != 0);
            if is_prime {
                primes.push(i);
            }
        }
        primes
    }

    fn factorize(mut n: u32) -> Vec<u32> {
        let primes = Self::primes_less_than(n);
        let mut factors = vec![1];
        while n != 1 {
            let divisor = primes.iter().copied().find(|prime| n % *prime == 0).unwrap();
            factors.push(divisor);
            n /= divisor;
        }
        factors
    }

    fn build(&self, rng: &mut dyn RngCore) -> (u32, String) {
        loop {
            let target = (1..40).choose(rng).unwrap();
            if let Some(question) = self.try_build(target, rng) {
                return (target, question)
            }
        }
    }

    fn try_build(&self, target: u32, rng: &mut dyn RngCore) -> Option<String> {
        match self {
            Expr::Num => {
                Some(format!("{target}"))
            }
            Expr::Add(left, right) => {
                let left_value = (1..target).choose(rng)?;
                let right_value = target - left_value;
                Some(format!("{} {} {}", left.try_build(left_value, rng)?, Self::OP_ADD, right.try_build(right_value, rng)?))
            }
            Expr::Sub(left, right) => {
                let right_value = (1..=15).choose(rng).unwrap();
                let left_value = target + right_value;
                Some(format!("{} {} {}", left.try_build(left_value, rng)?, Self::OP_SUB, right.try_build(right_value, rng)?))
            }
            Expr::Mul(left, right) => {
                let left_value;
                let right_value;
                if target == 1 {
                    left_value = 1;
                    right_value = 1;
                } else {
                    let mut factors = Self::factorize(target);
                    factors.shuffle(rng);
                    let partition = (1..factors.len()).choose(rng).unwrap();
                    left_value = factors[..partition].iter().product();
                    right_value = factors[partition..].iter().product();
                }
                Some(format!("{} {} {}", left.try_build(left_value, rng)?, Self::OP_MUL, right.try_build(right_value, rng)?))
            }
            Expr::Div(left, right) => {
                let right_value = (2..8).choose(rng).unwrap();
                let left_value = target * right_value;
                Some(format!("{} {} {}", left.try_build(left_value, rng)?, Self::OP_DIV, right.try_build(right_value, rng)?))
            }
        }
    }

    fn parse_add(ops: &mut Vec<&str>) -> Box<Expr> {
        let mut left = Self::parse_mul(ops);
        while !ops.is_empty() {
            let op = ops[0];
            match op {
                Self::OP_ADD => {
                    ops.remove(0);
                    left = Box::new(Expr::Add(left, Self::parse_mul(ops)));
                }
                Self::OP_SUB => {
                    ops.remove(0);
                    left = Box::new(Expr::Sub(left, Self::parse_mul(ops)));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }

    fn parse_mul(ops: &mut Vec<&str>) -> Box<Expr> {
        let mut left = Self::parse_num();
        while !ops.is_empty() {
            let op = ops[0];
            match op {
                Self::OP_MUL => {
                    ops.remove(0);
                    left = Box::new(Expr::Mul(left, Self::parse_num()));
                }
                Self::OP_DIV => {
                    ops.remove(0);
                    left = Box::new(Expr::Div(left, Self::parse_num()));
                }
                _ => {
                    break;
                }
            }
        }
        left
    }

    fn parse_num() -> Box<Expr> {
        Box::new(Expr::Num)
    }
}

pub struct ArithmeticProblem;

impl GenerateQuestion for ArithmeticProblem {
    fn generate(&self, rng: &mut dyn RngCore) -> Question {
        let term_count = (4..=6).choose(rng).unwrap();
        let mut terms: Vec<&'static str> = (0..term_count).map(|_| *Expr::OPS.choose(rng).unwrap()).collect();
        let (target, q) = Expr::parse_add(&mut terms).build(rng);

        let question = format!("What is {q}?");

        let validate = Box::new(move |answer: &str| {
            let answer= answer.parse::<u32>().map_err(|e| format!("Could not parse answer. ({})", e))?;
            if answer == target {
                Ok(())
            } else {
                Err(format!("The correct answer is {target}."))
            }
        });

        Question { text: question, check_answer: validate}
    }
}


#[cfg(test)]
mod test {
    use crate::arithmetic::Expr;

    #[test]
    fn test_primes() {
        for i in 1..50 {
            assert!(!Expr::primes_less_than(i).is_empty());
        }
    }

    #[test]
    fn test_factors() {
        for i in 1..50 {
            assert_eq!(Expr::factorize(i).into_iter().product::<u32>(), i);
        }
    }
}