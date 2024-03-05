use super::exp_tools::ExpTools;
use crate::hammer::tokenizer::include::{Token, TokenType};
use crate::tools::collections::Stack;
use std::collections::HashMap;
use crate::hammer::memory::Memory;
use super::decl_tools::DeclTools;

pub struct Tools {
    group_map: HashMap<TokenType, fn(&mut Tools, Token)>, 
    group_stack: Stack<fn(&mut Tools, Token)>,
    memory: Memory,
    exp_tools: ExpTools,
    decl_tools: DeclTools
}

impl Tools {
    pub fn new() -> Tools {
        Tools {
            group_map: init_group_map(),
            group_stack: Stack::new(),
            memory: Memory::new(),
            exp_tools: ExpTools::new(),
            decl_tools: DeclTools::new()
        }
    }

    pub fn tokenize(&mut self, token: Token) {
        self.group_stack.val()(self, token);
    }

    pub fn new_group(&mut self, type_token: TokenType) {
        self.group_stack.push(*self.group_map.get(&type_token).unwrap());
    }

    pub fn end_group(&mut self) {
        (self.group_stack.pop())(self, Token::new(TokenType::EndToken, String::new()));
    }

    fn expression(&mut self, token: Token) {
        match token.token_type {
            TokenType::Number => self.exp_tools.new_number(token.content),
            TokenType::Operator => self.exp_tools.new_operator(token.content),
            TokenType::Symbol => self.exp_tools.new_parenthesis(token.content),
            TokenType::EndToken => self.exp_tools.end(),
            _ => panic!("Unknow token type for an expression: {:?}    {}", token.token_type, token.content)
        }
    }
    
    fn declaration(&mut self, token: Token) {
        match token.token_type {
            TokenType::Type => self.decl_tools.def_type(token.content),
            TokenType::Ident => self.decl_tools.def_name(token.content, &mut self.memory),
            TokenType::Symbol => self.decl_tools.new_star(token.content),
            TokenType::Operator => todo!("Affectation not implemented yet."),
            TokenType::EndToken => self.decl_tools.end(),
            _ => panic!("Unknow token type for a declaration: {:?}    {}", token.token_type, token.content)
        }
    }
    fn keywordinst(&mut self, token: Token) {
        println!("Keywordinst: new token consumed: {}", token.content);
    }
    fn instruction(&mut self, token: Token) {
        println!("instruction: new token consumed: {}", token.content);
    }

    fn complex_ident(&mut self, token: Token) {
        println!("complexe ident: new token consumed: {}", token.content);  
    }
}


fn init_group_map() -> HashMap<TokenType, fn(&mut Tools, Token)> {
    let mut res = HashMap::<TokenType, fn(&mut Tools, Token)>::new();
    res.insert(TokenType::Instruction, Tools::instruction);
    res.insert(TokenType::Expression, Tools::expression);
    res.insert(TokenType::Declaration, Tools::declaration);
    res.insert(TokenType::IfKeyword, Tools::keywordinst);
    res.insert(TokenType::ForKeyword, Tools::keywordinst);
    res.insert(TokenType::WhileKeyword, Tools::keywordinst);
    res.insert(TokenType::DoKeyWord, Tools::keywordinst);
    res.insert(TokenType::FuncKeyword, Tools::keywordinst);
    res.insert(TokenType::ComplexIdent, Tools::complex_ident);
    res
}