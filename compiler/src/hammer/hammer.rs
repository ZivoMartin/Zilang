use std::thread::{JoinHandle, spawn};
use crate::tools::collections::Queue;
use super::tokenizer::{include::{Token, TokenType}, tokenizer::Tokenizer};
use std::process::exit;
use super::tools::program::Program;


pub struct Hammer {
    token_queue: Queue<Token>,
    tokenizing_thread: Vec<JoinHandle<()>>,
    keep_compile: bool,
    tools: Program,
}


impl<'a> Hammer {

    pub fn new() -> Hammer {
        Hammer{
            token_queue: Queue::new(),
            tokenizing_thread: Vec::new(),
            keep_compile: true,
            tools: Program::new()
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
                let token = self.token_queue.dequeue();
                match self.tools.tokenize(token) {
                    Ok(()) => (),
                    Err(e) => panic!("{e}")
                }
            }
        }
    }

    pub fn new_token(&mut self, token: Token) {
        self.token_queue.inqueue(token)
    }

    pub fn end_of_tokenizing_thread(&mut self) {
        self.keep_compile = false;
    }  

    pub fn new_group(&mut self, type_token: TokenType) {
        self.tools.new_group(type_token);
    }

    pub fn end_group(&mut self) {
        match self.tools.end_group() {
            Ok(()) => (),
            Err(e) => panic!("{e}")
        }
    }
}