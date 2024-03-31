use super::include::*;
use crate::zipiler::prog_manager::include::{ASM_SIZES, RAX_SIZE};
pub struct InstructionTools {
    /// The size of the raised memory spot
    size_left_ident: u8,
    /// Exemple: "+=", "-="...
    equal_code: String
}

impl Tool for InstructionTools {


    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
        match token.token_type {
            TokenType::Operator => self.set_equal_code(token.content),
            TokenType::RaiseExpression(_) => (),
            TokenType::MemorySpot(_, _, size) => self.set_ident(size),
            TokenType::RaiseDeclaration(_) | TokenType::MacroCall => (),
            TokenType::KeywordInstruction => (),
            TokenType::FuncCall(_) => (),
            TokenType::Keyword => res = self.new_kw(pm, token.content)?,
            _ => panic_bad_token("instruction", token)
        }
        Ok(res)
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(InstructionTools {
            size_left_ident: 0,
            equal_code: String::new()
        })
    }

    /// Build asm and simply return it
    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm();
        Ok((TokenType::Instruction, asm))
    }
}

impl InstructionTools {

    /// Called when we raise a memory spot, we just need to know the size of this memory spot. The address of the 
    /// memory spot is on the stack.
    fn set_ident(&mut self, size: u8) {
        self.size_left_ident = size;
    }

    /// Set the equal code of the affectation, exemple "+=" or "="
    fn set_equal_code(&mut self, equal_code: String) {
        self.equal_code = equal_code;
    }

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
    /// Put the reslt of the right expression on the address on the left.
    fn build_asm(&self) -> String {
        if self.size_left_ident != 0 {
            format!("
pop rax     ; result of the expression
pop rbx     ; addr of the left ident
{}",         format!("\n{} {}[_stack + rbx], {}", 
            match &self.equal_code as &str {
                "=" => "mov",
                "+=" => "add",
                "-=" => "sub",
                _ => panic!("Unknow equal code: {}", self.equal_code)
            },
            ASM_SIZES[self.size_left_ident as usize], 
            RAX_SIZE[self.size_left_ident as usize]))
        }else{
            String::new()
        }
    }
    
}