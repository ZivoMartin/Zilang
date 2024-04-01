use crate::zipiler::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct WhileTools{
    bi: u128
}

impl LoopTrait for WhileTools{
    fn bi(&self) -> u128 {
        self.bi
    }
}

impl Tool for WhileTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.jump_in();
        let res =WhileTools{
            bi: pm.bloc_id()
        };
        res.init(pm);
        Box::from(res)
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.end_loop(pm);
        Ok((TokenType::WhileKeyword, asm))
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::Keyword => res = self.new_keyword(&token.content, pm),
            TokenType::RaiseExpression(_) => res = self.compare_exp(pm),
            TokenType::Bloc | TokenType::Instruction => (),
            _ => pm.panic_bad_token("while keyword", token)
        }
        Ok(res)
    }
    
}
