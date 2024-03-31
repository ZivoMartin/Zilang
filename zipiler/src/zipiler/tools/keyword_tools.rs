
use super::include::*;


pub struct KeyWordTools {
    /// The saved script maybe has to be push at the end of the tool life
    save: String,
    /// The type of the keyword
    kw_type: TokenType
}

/// Uses for determinate on wich keyword we have to jump out
static DONT_JUMP_OUT_KW: [TokenType; 1] =  [TokenType::ReturnKeyword];

impl Tool for KeyWordTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(KeyWordTools{
            save: String::new(),
            kw_type: TokenType::KeywordInstruction
        })
    }

    /// We verify if the kw_word just found is a jumper keyword, if yes we jump out.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if !DONT_JUMP_OUT_KW.contains(&self.kw_type) {
            pm.jump_out();
        }
        Ok((TokenType::KeywordInstruction, String::new()))
    }   
    
    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::IfKeyword => res = self.if_keyword(pm, token.token_type),
            TokenType::WhileKeyword | 
            TokenType::ForKeyword |
            TokenType::FuncKeyword |
            TokenType::ReturnKeyword => res = self.end_kw(pm, token.token_type),
            TokenType::RaiseDoKeyWord(id) => self.end_do_kw(id),
            TokenType::RaiseExpression(_) => res = self.push_save(),
            _ => panic_bad_token("keyword inst", token)
        };
        Ok(res)
    }

}

impl KeyWordTools {


    /// Called when an if keyword just end, we simply reset the if_count and return the label for the global end
    /// of the if, uses for jump out of the bloc at the end of the selected one. Also call the end kw function 
    fn if_keyword(&mut self, pm: &mut ProgManager, tk_type: TokenType) -> String {
        pm.set_if_count(0);
        format!("\nglobal_end_if_{}:", pm.bloc_id())+&self.end_kw(pm, tk_type) 
    }

    /// Set the token type of this keyword instruction.
    fn end_kw(&mut self, _pm: &mut ProgManager, tk_type: TokenType) -> String{
        self.kw_type = tk_type;
        String::new()
    }

    /// Called when we catch an expression, we just, without thinking, push the saved script. Generally uses for the
    /// do while type of blocs. 
    fn push_save(&mut self) -> String {
        let res = self.save.clone();
        self.save = String::new();
        res
    }


    /// Called at the end of a do bloc. Maybe this bloc is going to be followed by a while keyword, if it is
    /// we ha to push a reloop script while the expression is true. But if we don't find any while we don't want
    /// to push this script so we just save the good script here, then if we catch an expression we push it.
    fn end_do_kw(&mut self, id: u128) {
        self.save = format!( "
pop rax
and rax, rax
jne begin_loop_{}", id);
    }

}