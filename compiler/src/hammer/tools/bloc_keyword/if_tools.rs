use crate::hammer::tools::include::*;

pub struct IfTools;

impl Tool for IfTools {

    fn new(_memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        Box::from(IfTools)
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(TokenType, String), String> {
        let asm = self.build_asm();
        Ok((TokenType::IfKeyword, asm))
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        Ok(match token.token_type {
            TokenType::Expression => self.compare_exp(memory),
            TokenType::Keyword => self.new_keyword(memory, token.content),
            TokenType::Bloc => self.end_bloc(memory),
            TokenType::Instruction | TokenType::IfKeyword => String::new(),
            _ => {panic_bad_token("if keyword", token);String::new()}
        })
    }
}

impl IfTools {

    fn new_keyword(&self, memory: &mut Memory, kw: String) -> String {
        if kw == "else" {
            println!("in");
            memory.jump_out();
            memory.jump_in();
        }
        String::new()
    }

    fn compare_exp(&self, memory: &Memory) -> String {
        format!("
pop rax
and rax, rax
je next_comp_if_{}_{}", memory.bloc_id, memory.if_count)
    }

    fn end_bloc(&mut self, memory: &mut Memory) -> String {
        let res = format!("
jmp global_end_if_{}
next_comp_if_{}_{}:", memory.bloc_id, memory.bloc_id, memory.if_count);
        memory.if_count += 1;
        res
    }

    fn build_asm(&self) -> String {
        String::new()
    }

}