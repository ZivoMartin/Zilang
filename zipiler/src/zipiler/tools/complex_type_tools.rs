use super::include::*;

pub struct ComplexTypeTools {
    stars: i32,
    name: String,
    size: u8
}

impl Tool for ComplexTypeTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(ComplexTypeTools{
            stars: 0,
            name: String::new(),
            size: 0
        })   
    }

    // Raise the type id, the number of stars and the size.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        Ok((TokenType::RaiseComplexType(pm.get_type_id_with_type_name(&self.name),  
                                        self.stars, self.size), String::new()))             
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        match token.token_type {
            TokenType::Symbol => self.new_star(),
            TokenType::Type => self.set_name(token.content, pm),
            _ => panic_bad_token("complex type", token)
        }
        Ok(String::new())
    }
}

impl ComplexTypeTools {

    fn new_star(&mut self) {
        self.size = 4;
        self.stars += 1;
    }

    fn set_name(&mut self, name: String, pm: &mut ProgManager) {
        self.name = name;
        self.size = pm.get_type_size(0, &self.name);
    }

}