use super::include::*;
pub struct InstructionTools;

impl Tool for InstructionTools {


    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
        match token.token_type {
            TokenType::Operator => (),
            TokenType::RaiseExpression(_) => (),
            TokenType::MemorySpot(_, _, _) => (),
            TokenType::RaiseDeclaration(_) | TokenType::MacroCall => (),
            TokenType::KeywordInstruction => (),
            TokenType::FuncCall(_) => (),
            TokenType::Keyword => res = self.new_kw(pm, token.content)?,
            _ => pm.panic_bad_token("instruction", token)
        }
        Ok(res)
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(InstructionTools)
    }

    /// Build asm and simply return it
    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm();
        Ok((TokenType::Instruction, asm))
    }
}

impl InstructionTools {


    /// Handle all the primitive keyword like break or continue, "return" has a special tree for him. 
    /// Can panic with an unknow kw
    fn new_kw(&self, _pm: &mut ProgManager, kw: String) -> Result<String, String> {
        match &kw as &str {
            "break" => todo!("Handle break"),
            "continue" => todo!("Handle continue"),
            _ => panic!("Unknow kw: {kw}")
        }
    }

    /// we start by retrieving the value from the stack of the right expression, then we we retrieve the address
    /// of the left spot. We now need two things, the size code (byte, dword..) depends 
    /// of the size_left_ident field and the mov code (add, mov..) depends of the equal code. 
    /// Put the result of the right expression at the address on the left.
    fn build_asm(&self) -> String {
        String::new()
    }
    
}