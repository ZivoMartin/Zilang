use super::include::*;

pub struct DeclTools {
    addr: usize,
    type_name: String,
    stars: i32,
    aff: bool
}

impl Tool for DeclTools {

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::Type => self.def_type(token.content),
            TokenType::Ident => self.def_name(token.content, memory),
            TokenType::Symbol => self.new_star(token.content),
            TokenType::Operator => self.def_equal_operator(),
            TokenType::Expression => (),
            _ => panic_bad_token("declaration", token)
        })
    }


    fn new() -> Box<dyn Tool> {
        Box::from(DeclTools {
            addr: 0,
            type_name: String::new(),
            stars: 0,
            aff: false
        })
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        let asm = self.build_asm(memory);
        Ok((Token::new(TokenType::Declaration, String::new()), asm))
    }

}

impl DeclTools {


    pub fn new_star(&mut self, content: String) {
        if content == "*" {
            self.stars += 1;
        }else{
            panic!("Bad symbol: {} when a star was expected", content);
        }
    }

    pub fn def_type(&mut self, t: String) {
        self.type_name = t;
    }

    pub fn def_name(&mut self, name: String, memory: &mut Memory) {
        self.addr = memory.new_var(self.type_name.clone(), name, self.stars);
    }

    pub fn def_equal_operator(&mut self) {
        self.aff = true;
    }

    fn build_asm(&self, memory: &mut Memory) -> String {
        let mut res = String::new();
        if self.aff {
            res.push_str("
pop rax"
           );
           res.push_str(&memory.affect_to(self.addr));
        }
        res
    }

}