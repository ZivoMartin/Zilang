use crate::hammer::tools::include::*;

use super::loop_trait::LoopTrait;



pub struct DoTools;

impl LoopTrait for DoTools{}

impl Tool for DoTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(DoTools)
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::Keyword => res = self.new_keyword(&token.content, pm),
            TokenType::Bloc => (),
            _ => panic_bad_token("do", token)
        }
        Ok(res)
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        Ok((TokenType::RaiseDoKeyWord(pm.bloc_id()), String::new()))
    }
}