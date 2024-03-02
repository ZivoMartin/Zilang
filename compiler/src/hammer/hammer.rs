use std::thread;

use super::tokenizer::tokenizer::Tokenizer;

pub struct Hammer;

impl Hammer {

    pub fn new() -> Hammer {
        Hammer
    }

    pub fn compile(&mut self, input: String) {
        let mut tokenizer = Tokenizer::new(self);
        match tokenizer.tokenize(input) {
            Ok(()) => (),
            Err(e) => panic!("{e}")
        }
    }

    pub fn new_token(&mut self, token: String) {
        println!("{}", token);
    }

}