use crate::{hammer::memory::Memory, tools::collections::Stack};
use std::collections::HashMap;
use crate::hammer::tokenizer::include::OPERATORS;
use super::program::{Tool, panic_bad_token};
use crate::hammer::tokenizer::include::{TokenType, Token};


pub struct ExpTools {
    op_stack: Stack<String>,
    pf_exp: Vec<ExpToken>,
    operator_priority: HashMap<String, u8>,
    op_id_map: HashMap<String, i64>,
    esp_decal: i64
}

impl Tool for ExpTools {
    
    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::Number => self.new_number(token.content),
            TokenType::Operator => self.new_operator(token.content),
            TokenType::Symbol => self.new_parenthesis(token.content),
            TokenType::ComplexIdent => self.new_cident(token.content),
            _ => panic_bad_token("expression", token)
        })
    }

    fn new() -> Box<dyn Tool> {
        Box::from(ExpTools{
            op_stack : Stack::new(),
            pf_exp : Vec::new(),
            operator_priority: build_prio_map(),
            op_id_map: build_op_id(),
            esp_decal: 0   
        })
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        while self.op_stack.size() != 0 {
            self.push_op_val();
        }
        let asm = build_asm(&self.pf_exp);
        Ok((Token::new(TokenType::Expression, String::new()), asm))
    }
    
}


impl ExpTools {


    pub fn new_operator(&mut self, content: String) {
        while !self.op_stack.is_empty() && self.op_stack.val() != "(" && self.get_priority(self.op_stack.val()) >= self.get_priority(&content){
            self.push_op_val();
        }
        self.op_stack.push(content);
    }

    pub fn new_number(&mut self, number: String) {
        self.pf_exp.push(ExpToken::new(ExpTokenType::Number, str::parse::<i64>(&number).unwrap()));
    }

    pub fn new_parenthesis(&mut self, par: String) {
        match &par as &str {
            "(" => self.op_stack.push(par),
            ")" => {
                while self.op_stack.val() != "(" {
                    self.push_op_val();
                }
                self.op_stack.pop();
            }
            _ => panic!("Unknow parenthesis: {par}")
        } 
    }

    pub fn new_cident(&mut self, size: String) {
        println!("{}", size);
        self.pf_exp.push(ExpToken::new(ExpTokenType::Ident, self.esp_decal));
        self.esp_decal += str::parse::<i64>(&size).unwrap();
    }

    fn get_priority(&self, op: &String) -> u8 {
        *self.operator_priority.get(op).unwrap_or_else(
            || panic!("This operator doesn't have priority yet: {op}")
        )
    }

    fn push_op_val(&mut self) {
        let val = *self.op_id_map.get(&self.op_stack.pop()).unwrap();
        self.pf_exp.push(ExpToken::new(ExpTokenType::Operator, val));    
    }

    fn _vec_exp(&self) -> Vec<i64> {
        let mut res = Vec::new();
        for t in self.pf_exp.iter() {
            res.push(t.content);
        }
        res
    }
}

fn build_prio_map() -> HashMap<String, u8>{
    let mut res = HashMap::<String, u8>::new();
    for op in vec!["%", "*", "/"].iter() {
        res.insert(String::from(*op), 4);
    }
    for op in vec!("<", "<=", ">", ">=", "==", "!=", "||", "&&").iter() {
        res.insert(String::from(*op), 2);
    }
    res.insert(String::from("+"), 3);
    res.insert(String::from("-"), 3);
    res.insert(String::from(")"), 4);
    res.insert(String::from("("), 5);
    res
}

fn build_op_id() -> HashMap<String, i64> {
    let mut res = HashMap::new();
    for (i, op) in OPERATORS.iter().enumerate() {
        res.insert(op.to_string(), i as i64);
    }
    res
}

enum ExpTokenType {
    Operator,
    Number,
    Ident
}

#[allow(dead_code)]
struct ExpToken {
    token_type: ExpTokenType,
    content: i64
}

impl ExpToken {
    fn new(token_type: ExpTokenType, content: i64) -> ExpToken {
        ExpToken{token_type, content}
    }
}

fn build_asm(pf: &Vec<ExpToken>) -> String {
    let mut res = String::from("\n");
    for t in pf.iter() {
        let n = t.content;
        match t.token_type {
            ExpTokenType::Operator => {
                res.push_str(&format!("
mov r12, {n}
pop r11
pop r10
call _operation
push rax"
                ))

            },
            ExpTokenType::Number => {
                res.push_str(&format!("
push {n}"
                ))
            },
            ExpTokenType::Ident => todo!("Aller recupérer la valeur sur la stack")
        }
    }
    res
}