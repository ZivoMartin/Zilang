use super::include::*;
pub struct InstructionTools;

impl Tool for InstructionTools {


    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::ComplexIdent => (),
            TokenType::Declaration | TokenType::MacroCall => (),
            _ => panic_bad_token("declaration", token)
        })
    }


    fn new() -> Box<dyn Tool> {
        Box::from(InstructionTools {
        })
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::Instruction, String::new()), String::new()))
    }
}