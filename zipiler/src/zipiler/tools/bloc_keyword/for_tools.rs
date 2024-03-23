use crate::zipiler::tools::include::*;

use super::loop_trait::LoopTrait;

pub struct ForTools {
    inst_number: u8
}

impl LoopTrait for ForTools{}

impl Tool for ForTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.jump_in();
        Box::from(ForTools{
            inst_number: 0
        })

    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.end_loop(pm);
        Ok((TokenType::ForKeyword, asm))
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Keyword => self.new_keyword(&token.content, pm),
            TokenType::RaiseExpression(_) => self.compare_exp(pm)            ,
            TokenType::Instruction => self.new_inst(pm),
            TokenType::Bloc => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}

impl ForTools {

    fn new_inst(&mut self, pm: &mut ProgManager) -> String {
        self.inst_number += 1;
        match self.inst_number {
            1 => format!("

jmp skip_first_loop_{id}
begin_loop_{id}:", id=pm.bloc_id()),
            
            2 => format!("\nskip_first_loop_{}:", pm.bloc_id()),
            _ => String::new()
        }
    }
}