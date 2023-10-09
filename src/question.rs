use rand::prelude::*;

pub trait CheckAnswer {
    fn check(&self, answer: &str) -> Result<(), String>;
}

impl<F> CheckAnswer for F where F: Fn(&str) -> Result<(), String> {
    fn check(&self, answer: &str) -> Result<(), String> {
        self(answer)
    }
}

pub struct Question {
    pub text: String,
    pub check_answer: Box<dyn CheckAnswer>
}

pub trait GenerateQuestion {
    fn generate(&self, rng: &mut dyn RngCore) -> Question;
}
