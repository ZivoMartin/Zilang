use crate::hammer::tools::include::*;

pub struct IfTools {
    id_bloc: u128,
    nb_cond: u16
}

impl Tool for IfTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        unsafe{
            BLOC_COUNT += 1;
            Box::from(IfTools{
                id_bloc: BLOC_COUNT,
                nb_cond: 0
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
            TokenType::Keyword => self.new_keyword(token.content),
            TokenType::Bloc => self.end_bloc(),
            TokenType::Instruction => String::new(),
            _ => {panic_bad_token("if keyword", token);String::new()}
        })
    }
}

impl IfTools {

    fn new_keyword(&self, _kw: String) -> String {
        String::new()
    }

    fn compare_exp(&self) -> String {
        format!("
pop rax
and rax, rax
je next_comp_if_{}_{}", self.id_bloc, self.nb_cond)
    }

    fn end_bloc(&mut self) -> String {
        let res = format!("
jmp global_end_if_{}:
next_comp_if_{}_{}:", self.id_bloc, self.id_bloc, self.nb_cond);
        self.nb_cond += 1;
        res
    }

    fn build_asm(&self) -> String {
        format!("
next_comp_if_{}_{}:
global_end_if_{}:", self.id_bloc, self.nb_cond, self.id_bloc)
    }

}