use std::thread::{JoinHandle, spawn};
use super::collections::Queue;
use super::tokenizer::{include::{Token, TokenType}, tokenizer::Tokenizer};
use super::program::Program;
use std::fs::File;
use super::prog_manager::include::{F_PATHS, files::*};
use std::io::{prelude::*, Write};
pub struct ZiLang {
    token_queue: Queue<Token>,
    tokenizing_thread: Vec<JoinHandle<()>>,
    keep_compile: bool,
    tools: Program,
    asm_files: Vec<File>,
}


impl<'a> ZiLang {

    pub fn new() -> ZiLang {
        ZiLang{
            token_queue: Queue::new(),
            tokenizing_thread: Vec::new(),
            keep_compile: true,
            tools: Program::new(),
            asm_files: open_asm_files(),
        }
    }

    pub fn compile(&mut self, input: File) -> Result<(), String>{
        let mut tokenizer = Tokenizer::new(self);
        self.tokenizing_thread.push(spawn(move || 
            _ = tokenizer.tokenize(input)
        ));
        while self.keep_compile || !self.token_queue.is_empty() {
            if !self.token_queue.is_empty() {
                let token = self.token_queue.dequeue().expect("Queue empty");
                let (asm, file_path) = self.tools.tokenize(token)?;
                self.push_script(&asm, file_path);
            }
        }
        self.tools.end_prog();
        Ok(())
    }

    pub fn new_token(&mut self, token: Token) {
        self.token_queue.inqueue(token)
    }

    pub fn end_of_tokenizing_thread(&mut self) {
        self.keep_compile = false;
    }  

    pub fn new_group(&mut self, type_token: TokenType) {
        self.new_token(Token::new_wflag(TokenType::New, String::new(), type_token));
    }

    pub fn end_group(&mut self) {
        self.new_token(Token::new(TokenType::End, String::new()));
    }
    
    fn push_script(&mut self, txt: &str, file_path: usize) {
        self.asm_files[file_path].write(txt.as_bytes()).unwrap();
    }

}




fn open_asm_files() -> Vec<File> {
    let mut res = Vec::<File>::new();
    for f in F_PATHS.iter() {
        res.push(File::options().append(true).read(true).open(f).unwrap_or_else(|e| {
            panic!("Failed to open the file {f}: {e}")
        }))
    }
    replace_txt(&mut res, SCRIPTF, BASE_SCRIPTF);
    res
}

fn replace_txt(arr: &mut Vec<File>, f1: usize, f2: usize) {
    arr[f1].set_len(0).unwrap();
    let mut s = String::new();
    arr[f2].read_to_string(&mut s).unwrap();
    arr[f1].write(s.as_bytes()).unwrap();
}