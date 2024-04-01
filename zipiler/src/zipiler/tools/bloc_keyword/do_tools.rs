use crate::zipiler::tools::include::*;

use super::loop_trait::LoopTrait;



pub struct DoTools {
    bi: u128
}

impl LoopTrait for DoTools{
    fn bi(&self) -> u128 {
        self.bi
    }
}

impl Tool for DoTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.jump_in();
        let res = DoTools{
            bi: pm.bloc_id()
        };
        res.init(pm);
        Box::from(res)
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::Keyword => res = self.new_keyword(&token.content, pm),
            TokenType::Bloc => (),
            _ => pm.panic_bad_token("do keyword", token)
        }
        Ok(res)
    }

    // Raise the id of the bloc
    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        Ok((TokenType::RaiseDoKeyWord(self.bi()), String::new()))
    }
}