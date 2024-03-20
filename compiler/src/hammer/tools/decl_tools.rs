use super::include::*;

pub struct DeclTools {
    addr: usize,
    type_name: String,
    stars: u32,
    aff: bool
}

impl Tool for DeclTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::ComplexType => self.def_type(token.content),
            TokenType::Ident => self.def_name(token.content, pm),
            TokenType::Operator => self.def_equal_operator(),
            TokenType::Expression => self.check_exp(token.content)?,
            _ => panic_bad_token("declaration", token)
        }
        Ok(String::new())
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(DeclTools {
            addr: 0,
            type_name: String::new(),
            stars: 0,
            aff: false
        })
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseDeclaration(self.addr), asm))
    }

}

impl DeclTools {



    pub fn def_type(&mut self, t: String) {
        let (name_t, stars, _) = extract_ctype_data(&t);
        self.type_name = name_t;
        self.stars = stars as u32;
    }

    pub fn check_exp(&mut self, stars: String) -> Result<(), String>{
        println!("{} {}", self.stars, stars);
        return if str::parse::<u32>(&stars).unwrap() != self.stars {
            Err(String::from("Not the good type"))
        }else{
            Ok(())
        }
    }

    pub fn def_name(&mut self, name: String, pm: &mut ProgManager) {
        self.addr = pm.new_var(self.type_name.clone(), name, self.stars);
    }

    pub fn def_equal_operator(&mut self) {
        self.aff = true;
    }

    fn build_asm(&self, pm: &mut ProgManager) -> String {
        let mut res = String::new();
        if self.aff {
            res.push_str("
pop rax"
           );
           res.push_str(&pm.affect_to(self.addr));
        }
        res
    }

}