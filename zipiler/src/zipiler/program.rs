use crate::zipiler::prog_manager::include::files::SCRIPTF;
use crate::zipiler::tokenizer::include::{Token, TokenType};
use super::collections::Stack;
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;
use super::prog_manager::prog_manager::ProgManager;
use super::prog_manager::include::{F_PATHS, files::*};
use std::io::{prelude::*, Write};
use super::tools::{
            exp_tools::ExpTools,
            decl_tools::DeclTools,
            cident_tools::CIdentTools,
            macrocall_tools::MacroCallTools,
            complexchar_tools::ComplexCharTools,
            instructions_tools::InstructionTools,
            keyword_tools::KeyWordTools,
            bloc_tools::BlocTools,
            complex_type_tools::ComplexTypeTools,
            bloc_keyword::{
                if_tools::IfTools,
                while_tools::WhileTools,
                for_tools::ForTools,
                do_tools::DoTools,
                func_tools::FuncTools,
                return_tools::ReturnTools
            }
            
        };

pub trait Tool {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized;

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String>;
    
    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>;

}



fn build_constructor_map() -> HashMap<TokenType, fn(&mut ProgManager) -> Box<dyn Tool>> {
    let mut res = HashMap::<TokenType, fn(pm: &mut ProgManager) -> Box<dyn Tool>>::new();
    res.insert(TokenType::Instruction, InstructionTools::new);
    res.insert(TokenType::Declaration, DeclTools::new);
    res.insert(TokenType::Expression, ExpTools::new);
    res.insert(TokenType::ComplexIdent, CIdentTools::new);
    res.insert(TokenType::MacroCall, MacroCallTools::new);
    res.insert(TokenType::ComplexType, ComplexTypeTools::new);
    res.insert(TokenType::ComplexChar, ComplexCharTools::new);
    res.insert(TokenType::Bloc, BlocTools::new);
    res.insert(TokenType::KeywordInstruction, KeyWordTools::new);
    res.insert(TokenType::IfKeyword, IfTools::new);
    res.insert(TokenType::ForKeyword, ForTools::new);
    res.insert(TokenType::WhileKeyword, WhileTools::new);
    res.insert(TokenType::FuncKeyword, FuncTools::new);
    res.insert(TokenType::DoKeyWord, DoTools::new);
    res.insert(TokenType::ReturnKeyword, ReturnTools::new);
    res
}

pub struct Program {
    memory: ProgManager,
    tools_stack: Stack<Box<dyn Tool>>,
    constructor_map: HashMap<TokenType, fn(pm: &mut ProgManager) -> Box<dyn Tool>>,
    line_number: u128,
    asm_files: Vec<File>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            memory: ProgManager::new(),
            tools_stack: Stack::new(),
            constructor_map: build_constructor_map(),
            line_number: 1,
            asm_files: open_asm_files(),
        }
    }

    // pub fn compile(&mut self) {
    //     loop {
    //         if !self.token_queue.is_empty() {
    //             let token = self.token_queue.dequeue().expect("Queue empty");
    //             let (asm, file_path) = self.tokenize(token).unwrap();
    //             self.push_script(&asm, file_path);
    //         }
    //     }
    // }

    pub fn tokenize(&mut self, token: Token) -> Result<(), String> {
        match token.token_type {
            TokenType::BackLine => self.new_line(),
            TokenType::ERROR => return Err(self.error_msg(token.content)),
            TokenType::End => self.end_group()?, 
            TokenType::New => self.new_group(token.flag),
            _ => {
                match self.tools_stack.val_mut().unwrap().new_token(token, &mut self.memory) {
                    Ok(asm) => self.push_script(&asm, SCRIPTF),
                    Err(e) => return Err(self.error_msg(e))
                }
            }
        };
        Ok(())
    }

    pub fn new_group(&mut self, type_token: TokenType) {
        // println!("new           {type_token:?}");
        self.tools_stack.push((self.constructor_map.get(&type_token).unwrap())(&mut self.memory));
    }

    pub fn end_group(&mut self) -> Result<(), String>{
        let (token_to_raise, end_txt) = self.tools_stack.pop().unwrap()
        .end(&mut self.memory).unwrap_or_else(|e| {
            println!("{}", self.error_msg(e));
            exit(1);
        });
        self.push_script(&end_txt, SCRIPTF);
        // println!("end           {:?}", token_to_raise);
        if !self.tools_stack.is_empty() {
            self.tokenize(Token::empty(token_to_raise))?;
        };
        Ok(())
    }

    pub fn _get_preload(&self) -> &String {
        &self.memory._get_preload()
    }

    pub fn end_prog(&mut self) {
        self.memory.end_prog();
    }

    fn new_line(&mut self) {
        self.line_number += 1;
    }

    fn error_msg(&self, msg: String) -> String {
        format!("{}: {}", self.line_number, msg)
    }
    
    fn push_script(&mut self, txt: &str, file_path: usize) {
        self.asm_files[file_path].write(txt.as_bytes()).unwrap();
    }

}


pub fn panic_bad_token(receiver: &str, token: Token) {
    panic!("Unknow token type for a {receiver}: {:?}    {}", token.token_type, token.content)
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