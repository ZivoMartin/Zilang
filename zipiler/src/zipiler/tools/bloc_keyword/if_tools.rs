use crate::zipiler::tools::include::*;

pub struct IfTools;

impl Tool for IfTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(IfTools)
    }

    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm();
        Ok((TokenType::IfKeyword, asm))
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::RaiseExpression(_) => self.compare_exp(pm),
            TokenType::Keyword => self.new_keyword(pm, token.content),
            TokenType::Bloc => self.end_bloc(pm),
            TokenType::Instruction | TokenType::IfKeyword => String::new(),
            _ => {panic_bad_token("if keyword", token);String::new()}
        })
    }
}

impl IfTools {

    fn new_keyword(&self, pm: &mut ProgManager, kw: String) -> String {
        if kw == "else" {
            println!("in");
            pm.jump_out();
            pm.jump_in();
        }
        String::new()
    }

    fn compare_exp(&self, pm: &ProgManager) -> String {
        format!("
pop rax
and rax, rax
je next_comp_if_{}_{}", pm.bloc_id(), pm.if_count())
    }

    fn end_bloc(&mut self, pm: &mut ProgManager) -> String {
        let res = format!("
jmp global_end_if_{}
next_comp_if_{}_{}:", pm.bloc_id(), pm.bloc_id(), pm.if_count());
        pm.inc_if_count();
        res
    }

    fn build_asm(&self) -> String {
        String::new()
    }

}