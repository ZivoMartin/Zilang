use super::exp_tools::ExpTools;
use crate::hammer::tokenizer::include::{Token, TokenType};
use crate::tools::collections::Stack;
use std::collections::HashMap;
use crate::hammer::memory::Memory;
use super::decl_tools::DeclTools;
use super::cident_tools::CIdentTools;

pub trait Tool {

    fn new() -> Box<dyn Tool> where Self: Sized;
    
    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<Option<Token>, String>;

}

fn build_constructor_map() -> HashMap<TokenType, fn() -> Box<dyn Tool>> {
    let mut res = HashMap::<TokenType, fn() -> Box<dyn Tool>>::new();
    res.insert(TokenType::Declaration, DeclTools::new);
    res.insert(TokenType::Expression, ExpTools::new);
    res.insert(TokenType::ComplexIdent, CIdentTools::new);
    res
}

pub struct Program {
    memory: Memory,
    tools_stack: Stack<Box<dyn Tool>>,
    constructor_map: HashMap<TokenType, fn() -> Box<dyn Tool>>
}

impl Program {
    pub fn new() -> Program {
        Program {
            memory: Memory::new(),
            tools_stack: Stack::new(),
            constructor_map: build_constructor_map()
        }
    }

    pub fn tokenize(&mut self, token: Token) -> Result<(), String> {
        self.tools_stack.val_mut().new_token(token, &mut self.memory)?;
        Ok(())
    }

    pub fn new_group(&mut self, type_token: TokenType) {
        self.tools_stack.push((self.constructor_map.get(&type_token).unwrap())());
    }

    pub fn end_group(&mut self) -> Result<(), String>{
        self.tools_stack.pop().new_token(Token::new(TokenType::EndToken, String::new()), &mut self.memory)?;
        Ok(())
    }

    // pub fn raise_result(&mut self, token_type: TokenType, content: String) -> Result<(), String>{
    //     (self.group_stack.val())(self, Token::new(token_type, content))
    // }
}


pub fn panic_bad_token(receiver: &str, token: Token) {
    panic!("Unknow token type for a {receiver}: {:?}    {}", token.token_type, token.content)
}

