
use super::include::*;

pub struct KeyWordTools;

impl Tool for KeyWordTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        Box::from(KeyWordTools{
        })
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::KeywordInstruction, String::new()), String::new()))
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        let res = match token.token_type {
            TokenType::IfKeyword => self.if_keyword(memory),
            TokenType::WhileKeyword | TokenType::ForKeyword | TokenType::Bloc => String::new(),
            _ => {panic_bad_token("keyword inst", token);String::new()}
        };
        Ok(res)
    }

}

impl KeyWordTools {

    fn if_keyword(&self, memory: &mut Memory) -> String {
        memory.if_count = 0;
        format!("\nglobal_end_if_{}:", memory.bloc_id)
    }

}