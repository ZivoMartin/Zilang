use crate::zipiler::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct WhileTools;

impl LoopTrait for WhileTools{}

impl Tool for WhileTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.jump_in();
        Box::from(WhileTools)
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.end_loop(pm);
        Ok((TokenType::WhileKeyword, asm))
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Keyword => self.new_keyword(&token.content, pm),
            TokenType::RaiseExpression(_) => self.compare_exp(pm),
            TokenType::Bloc | TokenType::Instruction => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}
