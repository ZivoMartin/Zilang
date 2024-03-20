use super::include::*;
pub struct BlocTools;

impl Tool for BlocTools {


    fn new_token(&mut self, token: Token, _pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::Instruction => (),
            _ => panic_bad_token("bloc", token)
        }
        Ok(String::new())
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(BlocTools)
    }

    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        Ok((TokenType::Bloc, String::new()))
    }
}