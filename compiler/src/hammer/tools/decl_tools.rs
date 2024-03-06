use crate::hammer::memory::Memory;
use super::program::{Tool, panic_bad_token};
use crate::hammer::tokenizer::include::{TokenType, Token};

pub struct DeclTools {
    name: String,
    type_name: String,
    stars: i32,
    equal_op: String
}

impl Tool for DeclTools {

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<Option<Token>, String>{
        match token.token_type {
            TokenType::Type => self.def_type(token.content),
            TokenType::Ident => self.def_name(token.content, memory),
            TokenType::Symbol => self.new_star(token.content),
            TokenType::Operator => self.def_equal_operator(token.content),
            TokenType::Expression => (),
            TokenType::EndToken => self.end(memory),
            _ => panic_bad_token("declaration", token)
        }
        Ok(None)
    }


    fn new() -> Box<dyn Tool> {
        Box::from(DeclTools {
            name: String::new(),
            type_name: String::new(),
            stars: 0,
            equal_op: String::new()
        })
    }

}

impl DeclTools {


    pub fn new_star(&mut self, content: String) {
        if content == "*" {
            self.stars += 1;
        }else{
            panic!("Bad symbol: {} when a star was expected", content);
        }
    }

    pub fn def_type(&mut self, t: String) {
        self.type_name = t;
    }

    pub fn def_name(&mut self, name: String, memory: &mut Memory) {
        self.name = name;
        memory.new_var(self.type_name.clone(), self.name.clone(), self.stars);
    }

    pub fn def_equal_operator(&mut self, op: String) {
        self.equal_op = op;
    }


    pub fn end(&mut self, _memory: &mut Memory) {
        if !self.equal_op.is_empty() {
            todo!("Implementer les affectations");
        }
        self.name.clear();
        self.type_name.clear();
        self.equal_op.clear();
        self.stars = 0;
    }

}