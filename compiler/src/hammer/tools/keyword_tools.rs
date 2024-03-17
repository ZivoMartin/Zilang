
use super::include::*;

pub struct KeyWordTools {
    save: String
}

impl Tool for KeyWordTools {

    fn new(memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        memory.jump_in();
        Box::from(KeyWordTools{
            save: String::new()
        })
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        memory.jump_out();
        Ok((Token::new(TokenType::KeywordInstruction, String::new()), String::new()))
    }   

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        let res = match token.token_type {
            TokenType::IfKeyword => self.if_keyword(memory),
            TokenType::WhileKeyword | 
            TokenType::ForKeyword |
            TokenType::FuncKeyword => self.end_kw(memory),
            TokenType::DoKeyWord => self.end_do_kw(token.content),
            TokenType::Expression => self.push_save(),
            _ => {panic_bad_token("keyword inst", token);String::new()}
        };
        Ok(res)
    }

}

impl KeyWordTools {

    fn if_keyword(&self, memory: &mut Memory) -> String {
        memory.if_count = 0;
        format!("\nglobal_end_if_{}:", memory.bloc_id)+&self.end_kw(memory) 
    }

    fn end_kw(&self, memory: &mut Memory) -> String{
        memory.bloc_id += 1;
        String::new()
    }

    fn push_save(&mut self) -> String {
        let res = self.save.clone();
        self.save = String::new();
        res
    }

    fn end_do_kw(&mut self, save: String) -> String {
        self.save = save;
        String::new()
    }
}