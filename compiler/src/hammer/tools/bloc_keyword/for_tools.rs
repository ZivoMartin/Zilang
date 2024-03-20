use crate::hammer::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct ForTools {
    inst_number: u8
}

impl LoopTrait for ForTools{}

impl Tool for ForTools {

    fn new(_memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        Box::from(ForTools{
            inst_number: 0
        })

    }

    fn end(&mut self, memory: &mut Memory) -> Result<(TokenType, String), String> {
        let asm = self.end_loop(memory);
        Ok((TokenType::ForKeyword, asm))
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Keyword => self.new_keyword(&token.content, memory),
            TokenType::Expression => self.compare_exp(memory)            ,
            TokenType::Instruction => self.new_inst(memory),
            TokenType::Bloc => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}

impl ForTools {

    fn new_inst(&mut self, memory: &mut Memory) -> String {
        self.inst_number += 1;
        match self.inst_number {
            1 => format!("

jmp skip_first_loop_{id}
begin_loop_{id}:", id=memory.bloc_id),
            
            2 => format!("\nskip_first_loop_{}:", memory.bloc_id),
            _ => String::new()
        }
    }
}