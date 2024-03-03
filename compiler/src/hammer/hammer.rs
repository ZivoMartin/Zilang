use std::thread::{JoinHandle, spawn};
use crate::tools::collections::Queue;
use super::tokenizer::{include::{Token, TokenType}, tokenizer::Tokenizer};
use std::process::exit;

pub struct Hammer {
    token_queue: Queue<Token>,
    tokenizing_thread: Vec<JoinHandle<()>>,
    keep_compile: bool
}



impl<'a> Hammer {

    pub fn new() -> Hammer {
        Hammer{
            token_queue: Queue::new(),
            tokenizing_thread: Vec::new(),
            keep_compile: true
        }
    }

    pub fn compile(&mut self, input: String) {
        let mut tokenizer = Tokenizer::new(self);
        self.tokenizing_thread.push(spawn(move || 
            tokenizer.tokenize(input).unwrap_or_else(|e| {
                eprintln!("{e}");
                exit(1);
            })
        ));
        while self.keep_compile {
            if !self.token_queue.is_empty() {
                self.token_queue.dequeue();
                //println!("NEW TOKEN INTERCEPTED: {}", self.token_queue.dequeue().content)
            }
        }
    }

    pub fn new_token(&mut self, token: Token) {
        println!("NEW TOKEN INTERCEPTED: {}", token.content);
        self.token_queue.inqueue(token)
    }

    pub fn end_of_tokenizing_thread(&mut self) {
        self.keep_compile = false;
    }  

    pub fn new_group(&mut self, type_token: TokenType) {
        println!("NEW GROUP: {:?}", type_token)
    }

    pub fn end_group(&mut self, type_token: TokenType) {
        println!("END GROUP: {type_token:?}\n")
    }

}