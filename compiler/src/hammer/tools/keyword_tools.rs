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

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<String, String> {
        match token.token_type {
            TokenType::IfKeyword | TokenType::Bloc => (),
            _ => panic_bad_token("keyword inst", token)
        }
        Ok(String::new())
    }

}