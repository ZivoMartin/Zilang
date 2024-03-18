use super::include::*;

pub struct ComplexTypeTools {
    stars: u8,
    name: String,
    size: u32
}

impl Tool for ComplexTypeTools {

    fn new(memory: &mut Memory) -> Box<dyn Tool> where Self: Sized {
        
    }

    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        
    }

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<String, String> {
        
        Ok(String::new())
    }
}