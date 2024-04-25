use crate::hammer::tools::include::*;

pub struct ForTools {
    _id_bloc: u128
}

impl Tool for ForTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        unsafe{
            BLOC_COUNT += 1;
            Box::from(ForTools{
                _id_bloc: BLOC_COUNT,
            })
        }
        
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        let asm = self.build_asm();
        Ok((Token::new(TokenType::IfKeyword, String::new()), asm))
    }

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            _ => {panic_bad_token("if keyword", token);String::new()}
        })
    }
    
}

impl ForTools {

    fn build_asm(&self) -> String {
        String::new()
    }

}