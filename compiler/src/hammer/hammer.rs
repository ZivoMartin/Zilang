use std::thread::{JoinHandle, spawn};
use crate::tools::collections::{Queue, Stack};
use std::collections::HashMap;
use super::tokenizer::{include::{Token, TokenType}, tokenizer::Tokenizer};
use std::process::exit;


pub struct Hammer {
    token_queue: Queue<Token>,
    tokenizing_thread: Vec<JoinHandle<()>>,
    keep_compile: bool,
    group_map: HashMap<TokenType, fn(&mut Hammer, Token)>, 
    group_stack: Stack<fn(&mut Hammer, Token)>
}


fn init_group_map() -> HashMap<TokenType, fn(&mut Hammer, Token)> {
    let mut res = HashMap::<TokenType, fn(&mut Hammer, Token)>::new();
    res.insert(TokenType::Instruction, Hammer::instruction);
    res.insert(TokenType::Expression, Hammer::expression);
    res.insert(TokenType::Declaration, Hammer::declaration);
    res.insert(TokenType::IfKeyword, Hammer::keywordinst);
    res.insert(TokenType::ForKeyword, Hammer::keywordinst);
    res.insert(TokenType::WhileKeyword, Hammer::keywordinst);
    res.insert(TokenType::DoKeyWord, Hammer::keywordinst);
    res.insert(TokenType::FuncKeyword, Hammer::keywordinst);
    res
}


impl<'a> Hammer {

    pub fn new() -> Hammer {
        Hammer{
            token_queue: Queue::new(),
            tokenizing_thread: Vec::new(),
            keep_compile: true,
            group_map: init_group_map(),
            group_stack: Stack::new()
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
                self.group_stack.val()(self, token);
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
        println!("\nNEW GROUP: {:?}", type_token);
        self.group_stack.push(*self.group_map.get(&type_token).unwrap());
    }

    pub fn end_group(&mut self, type_token: TokenType) {
        self.group_stack.pop();
        println!("END GROUP: {type_token:?}\n")
    }

    fn expression(&mut self, token: Token) {
        println!("Expression: new token consumed: {}", token.content);
    }
    fn declaration(&mut self, token: Token) {
        println!("Declaration: new token consumed: {}", token.content);
    }
    fn keywordinst(&mut self, token: Token) {
        println!("Keywordinst: new token consumed: {}", token.content);
    }
    fn instruction(&mut self, token: Token) {
        println!("instruction: new token consumed: {}", token.content);
    }


}