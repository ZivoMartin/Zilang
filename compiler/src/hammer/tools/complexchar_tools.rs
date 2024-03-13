use super::include::*;

pub struct ComplexCharTools {
    bs: bool,
    symb: char
}

impl Tool for ComplexCharTools {

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content.chars().next().unwrap()),
            _ => panic_bad_token("complex ident", token)
        })
    }
    
    fn new() -> Box<dyn Tool> {
        Box::from(ComplexCharTools {
            bs: false,
            symb: '\0'
        })
    }

   
    fn end(&mut self, _memory: &mut Memory) -> Result<(Token, String), String> {
        Ok((Token::new(TokenType::ComplexChar, 
            if self.bs {
                match self.symb {
                    '0' => "0",
                    't' => "9",
                    'n' => "10",
                    'r' => "13",
                    '\"' => "34",
                    '\'' => "39",
                    '\\' => "92",
                    _ => return Err(format!("This char: \\{} doesn't exists.", self.symb))
                }.to_string()
            }else{
                format!("{}", self.symb as u8)
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