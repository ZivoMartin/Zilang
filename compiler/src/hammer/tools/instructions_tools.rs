use super::include::*;
use crate::hammer::prog_manager::include::{ASM_SIZES, RAX_SIZE};
pub struct InstructionTools {
    size_aff: u8,
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
            size_aff: 0,
            equal_code: String::new()
        })
    }

    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm();
        Ok((TokenType::Instruction, asm))
    }
}

impl InstructionTools {

    fn set_ident(&mut self, size: u8) {
        self.size_aff = size;
    }

    fn set_equal_code(&mut self, equal_code: String) {
        self.equal_code = equal_code;
    }

    fn new_kw(&self, _pm: &mut ProgManager, kw: String) -> Result<String, String> {
        match &kw as &str {
            "break" => todo!("Handle break"),
            "continue" => todo!("Handle continue"),
            _ => panic!("Unknow kw: {kw}")
        }
    }


    fn build_asm(&self) -> String {
        if self.size_aff != 0 {
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
            ASM_SIZES[self.size_aff as usize], RAX_SIZE[self.size_aff as usize]))
        }else{
            String::new()
        }
    }
    
}