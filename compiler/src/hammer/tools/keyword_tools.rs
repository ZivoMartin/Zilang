
use super::include::*;

pub struct KeyWordTools {
    save: String
}

impl Tool for KeyWordTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.jump_in();
        Box::from(KeyWordTools{
            save: String::new()
        })
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        pm.jump_out();
        Ok((TokenType::KeywordInstruction, String::new()))
    }   

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let res = match token.token_type {
            TokenType::IfKeyword => self.if_keyword(pm),
            TokenType::WhileKeyword | 
            TokenType::ForKeyword |
            TokenType::FuncKeyword => self.end_kw(pm),
            TokenType::RaiseDoKeyWord(id) => self.end_do_kw(id),
            TokenType::Expression => self.push_save(),
            _ => {panic_bad_token("keyword inst", token);String::new()}
        };
        Ok(res)
    }

}

impl KeyWordTools {

    fn if_keyword(&self, pm: &mut ProgManager) -> String {
        pm.set_if_count(0);
        format!("\nglobal_end_if_{}:", pm.bloc_id())+&self.end_kw(pm) 
    }

    fn end_kw(&self, pm: &mut ProgManager) -> String{
        pm.inc_bi();
        String::new()
    }

    fn push_save(&mut self) -> String {
        let res = self.save.clone();
        self.save = String::new();
        res
    }

    fn end_do_kw(&mut self, id: u128) -> String {
        self.save = format!( "pop rax
            and rax, rax
            jne begin_loop_{}", id);
        String::new()
    }
}