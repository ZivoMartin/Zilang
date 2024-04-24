use super::include::*;
pub struct BlocTools;

impl Tool for BlocTools {


    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<String, String>{
        match token.token_type {
            TokenType::Instruction => println!("receiv"),
            _ => panic_bad_token("bloc", token)
        }
        Ok(String::new())
    }


    fn new() -> Box<dyn Tool> {
        Box::from(BlocTools {
        })
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::Instruction, String::new()), String::new()))
    }
}