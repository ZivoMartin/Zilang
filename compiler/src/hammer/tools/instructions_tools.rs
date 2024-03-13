use super::include::*;
pub struct InstructionTools;

impl Tool for InstructionTools {


    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::ComplexIdent => setup_aff()
            TokenType::Declaration | TokenType::MacroCall => (),
            _ => panic_bad_token("declaration", token)
        })
    }


    fn new() -> Box<dyn Tool> {
        Box::from(InstructionTools {
        })
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::Instruction, String::new()), String::new()))
    }
}