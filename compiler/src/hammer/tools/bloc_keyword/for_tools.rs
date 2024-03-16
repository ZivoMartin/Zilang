use crate::hammer::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct ForTools {
    inst_number: u8
}

impl LoopTrait for ForTools{}

impl Tool for ForTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        Box::from(ForTools{
            inst_number: 0
        })

    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        let asm = self.end_loop(memory);
        Ok((Token::new(TokenType::ForKeyword, String::new()), asm))
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Keyword => self.new_keyword(&token.content, memory),
            TokenType::Expression => self.compare_exp(memory),
            TokenType::Instruction => self.new_inst(memory),
            TokenType::Bloc => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}

impl ForTools {

    fn new_inst(&mut self, memory: &mut Memory) -> String {
        inst_number += 1;
        match self.inst_number {
            1 => format!("\nbegin_loop_{}:", memory.bloc_id),
            2 => 
            _ => String::new()
        }
            self.inst_number = false;
            
        }else{
            String::new()
        }
    }

}