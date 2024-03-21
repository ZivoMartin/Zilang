use super::include::*;

pub struct ComplexCharTools {
    bs: bool,
    symb: char
}

impl Tool for ComplexCharTools {

    fn new_token(&mut self, token: Token, _pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content.chars().next().unwrap()),
            _ => panic_bad_token("complex ident", token)
        }
        Ok(String::new())
    }
    
    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(ComplexCharTools {
            bs: false,
            symb: '\0'
        })
    }

   // We raise the ascii value of the char
    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        Ok((TokenType::RaiseComplexChar( 
            if self.bs {
                match self.symb {
                    '0' => 0,
                    't' => 9,
                    'n' => 10,
                    'r' => 13,
                    '\"' => 34,
                    '\'' => 39,
                    '\\' => 92,
                    _ => return Err(format!("This char: \\{} doesn't exists.", self.symb))
                }
            }else{
                self.symb as u8
            }
        ), String::new()))
        
    }
}


impl ComplexCharTools {

    fn new_symbol(&mut self, symb: char) {
        if symb == '\\' && !self.bs {
            self.bs = true
        }else{
            self.symb = symb
        }
    }

}