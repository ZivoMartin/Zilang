use crate::hammer::tools::include::*;

use super::loop_trait::LoopTrait;



pub struct DoTools;

impl LoopTrait for DoTools{}

impl Tool for DoTools {

    fn new(_memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        Box::from(DoTools)
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::Keyword => res = self.new_keyword(&token.content, memory),
            TokenType::Bloc => (),
            _ => panic_bad_token("do", token)
        }
        Ok(res)
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::DoKeyWord, format!("
        pop rax
        and rax, rax
        jne begin_loop_{}", memory.bloc_id)), String::new()))
    }
}