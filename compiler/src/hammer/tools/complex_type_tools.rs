use super::include::*;

pub struct ComplexTypeTools {
    stars: i32,
    name: String,
    size: u8
}

impl Tool for ComplexTypeTools {

    fn new(_memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        Box::from(ComplexTypeTools{
            stars: 0,
            name: String::new(),
            size: 0
        })   
    }

    fn end(&mut self, _memory: &mut Memory) -> Result<(TokenType, String), String> {
        Ok((TokenType::RaiseComplexType( 
            Type::new(self.name.clone(), self.size, self.stars)), String::new()))             
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        match token.token_type {
            TokenType::Symbol => self.new_star(),
            TokenType::Type => self.set_name(token.content, memory),
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

    fn set_name(&mut self, name: String, memory: &mut Memory) {
        self.name = name;
        self.size = memory.get_type_size(0, &self.name);
    }

}