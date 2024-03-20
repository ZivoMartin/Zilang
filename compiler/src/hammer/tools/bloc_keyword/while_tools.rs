use crate::hammer::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct WhileTools;

impl LoopTrait for WhileTools{}

impl Tool for WhileTools {

    fn new(_memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        Box::from(WhileTools)
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(TokenType, String), String> {
        let asm = self.end_loop(memory);
        Ok((TokenType::WhileKeyword, asm))
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Keyword => self.new_keyword(&token.content, memory),
            TokenType::Expression => self.compare_exp(memory),
            TokenType::Bloc | TokenType::Instruction => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}
