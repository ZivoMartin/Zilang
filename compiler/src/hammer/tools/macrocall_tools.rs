use super::include::*;

pub struct MacroCallTools {
    name: String,
    nb_param: u8,
    nb_param_attempt: u8
}

static MACRO_LIST: [(&str, u8); 3] = [("dn", 1), ("exit", 1), ("print_char", 1)];

static _SIZE_PARAM: u8 = 8;

impl Tool for MacroCallTools {

    fn new() -> Box<dyn Tool> where Self: Sized {
        Box::from(MacroCallTools{
            name: String::new(),
            nb_param: 0,
            nb_param_attempt: 0
        })
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        if self.nb_param != self.nb_param_attempt {
            Err(format!("{} args has been found for the macro {} when {} was attempts", self.nb_param, self.name, self.nb_param_attempt))
        }else{
            let asm = self.build_asm();
            Ok((Token::new(TokenType::MacroCall, String::new()), asm))
        }
    }

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<String, String> {
        match token.token_type {
            TokenType::Ident => self.def_name(token.content)?,
            TokenType::Expression => self.new_expression(),
            TokenType::ExpressionTuple => (),
            _ => panic_bad_token("macro call", token)
        }
        Ok(String::new())
    }

}

impl MacroCallTools {

    fn def_name(&mut self, name: String) -> Result<(), String> {
        for m in MACRO_LIST.iter() {
            if m.0 == &name {
                self.name = m.0.to_string();
                return Ok(self.nb_param_attempt = m.1)
            }
        }
        Err(format!("Unknow macro: {}", name))
    }

    fn new_expression(&mut self) {
        self.nb_param += 1;
    }

    fn build_asm(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!("\n{} ", self.name));
        for i in (0..self.nb_param).rev() {
            res.push_str(&format!(
                "[rsp + {}]", i*8
            ))
        }
        res.push_str(&format!("\nadd rsp, {}", self.nb_param*8));
        res
    }

}