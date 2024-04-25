use crate::hammer::tools::include::*;

pub struct WhileTools {
    id_bloc: u128
}

impl Tool for WhileTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        unsafe{
            BLOC_COUNT += 1;
            Box::from(WhileTools{
                id_bloc: BLOC_COUNT,
            })
        }
        
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        let asm = self.build_asm();
        Ok((Token::new(TokenType::IfKeyword, String::new()), asm))
    }

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Expression => self.compare_exp(),
            TokenType::Bloc | TokenType::Instruction => String::new(),
            _ => {panic_bad_token("while keyword", token);String::new()}
        })
    }
    
}

impl WhileTools {

    fn compare_exp(&self) -> String {
        format!("
pop rax
and rax, rax
je end_while_{}", self.id_bloc)
    }

    fn build_asm(&self) -> String {
        format!("
end_while_{}", self.id_bloc)
    }

}