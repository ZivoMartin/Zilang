use super::exp_tools::ExpTools;
use crate::hammer::tokenizer::include::{Token, TokenType};
use crate::tools::collections::Stack;
use std::collections::HashMap;
use crate::hammer::memory::Memory;
use super::decl_tools::DeclTools;
use super::cident_tools::CIdentTools;
use super::macrocall_tools::MacroCallTools;

pub trait Tool {

    fn new() -> Box<dyn Tool> where Self: Sized;

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String>;
    
    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<(), String>;

}

fn build_constructor_map() -> HashMap<TokenType, fn() -> Box<dyn Tool>> {
    let mut res = HashMap::<TokenType, fn() -> Box<dyn Tool>>::new();
    res.insert(TokenType::Declaration, DeclTools::new);
    res.insert(TokenType::Expression, ExpTools::new);
    res.insert(TokenType::ComplexIdent, CIdentTools::new);
    res.insert(TokenType::MacroCall, MacroCallTools::new);
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

    pub fn end_group(&mut self) -> Result<String, String>{
        let (token_to_raise, end_txt) = self.tools_stack.pop().end(&mut self.memory)?;
        if !self.tools_stack.is_empty() {
            self.tokenize(token_to_raise)?;
        }
        Ok(end_txt)
    }

    
}


pub fn panic_bad_token(receiver: &str, token: Token) {
    panic!("Unknow token type for a {receiver}: {:?}    {}", token.token_type, token.content)
}

