
use super::include::*;

pub struct KeyWordTools {
    save: String,
    kw_type: TokenType
}

static DONT_JUMP_OUT_KW: [TokenType; 1] =  [TokenType::ReturnKeyword];

impl Tool for KeyWordTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(KeyWordTools{
            save: String::new(),
            kw_type: TokenType::KeywordInstruction
        })
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if !DONT_JUMP_OUT_KW.contains(&self.kw_type) {
            pm.jump_out();
        }
        Ok((TokenType::KeywordInstruction, String::new()))
    }   

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        self.kw_type = token.token_type;
        match token.token_type {
            TokenType::IfKeyword => res = self.if_keyword(pm),
            TokenType::WhileKeyword | 
            TokenType::ForKeyword |
            TokenType::FuncKeyword |
            TokenType::ReturnKeyword => res = self.end_kw(pm),
            TokenType::RaiseDoKeyWord(id) => self.end_do_kw(id),
            TokenType::RaiseExpression(_) => res = self.push_save(),
            _ => panic_bad_token("keyword inst", token)
        };
        Ok(res)
    }

}

impl KeyWordTools {

    fn if_keyword(&self, pm: &mut ProgManager) -> String {
        pm.set_if_count(0);
        format!("\nglobal_end_if_{}:", pm.bloc_id())+&self.end_kw(pm) 
    }

    fn end_kw(&self, _pm: &mut ProgManager) -> String{
        String::new()
    }

    fn push_save(&mut self) -> String {
        let res = self.save.clone();
        self.save = String::new();
        res
    }

    fn end_do_kw(&mut self, id: u128) {
        self.save = format!( "
pop rax
and rax, rax
jne begin_loop_{}", id);
    }

}