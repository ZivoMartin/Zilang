use super::include::*;
use crate::hammer::prog_manager::prog_manager::{ASM_SIZES, RAX_SIZE};
pub struct InstructionTools {
    size_aff: u32,
    equal_code: String
}

impl Tool for InstructionTools {


    fn new_token(&mut self, token: Token, _pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::Operator => self.set_equal_code(token.content),
            TokenType::Expression => (),
            TokenType::ComplexIdent => self.set_ident(token.content),
            TokenType::Declaration | TokenType::MacroCall => (),
            TokenType::KeywordInstruction => (),
            _ => panic_bad_token("instruction", token)
        }
        Ok(String::new())
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

    pub fn set_ident(&mut self, cident_data: String) {
        let (_, _, size) = extract_cident_data(&cident_data);
        self.size_aff = size;
    }

    pub fn set_equal_code(&mut self, equal_code: String) {
        self.equal_code = equal_code;
    }

    pub fn build_asm(&self) -> String {
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