use crate::hammer::tools::include::*;


pub struct ReturnTools {
    stars: i32
}


impl Tool for ReturnTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(ReturnTools{
            stars: -1
        })
    }

    fn new_token(&mut self, token: Token, _pm: &mut ProgManager) -> Result<String, String> {
        let res = String::new();
        match token.token_type {    
            TokenType::RaiseExpression(stars) => self.set_exp(stars),
            _ => panic_bad_token("return keyword", token)
        }
        Ok(res)
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = String::from("
pop rax
ret");
        let cf = pm.current_func(); 
        if !cf.check_valid_return_type(self.stars) {
            Err(format!("The function {} return a value with {} stars, you returned {}", 
            cf.name(), cf.return_type().stars(), self.stars))
        }else if !pm.is_in_func() {
            Err(String::from("You tried to return a value without being in a function"))
        }else{
            Ok((TokenType::ReturnKeyword, asm))
        }
    }   

}

impl ReturnTools {
    fn set_exp(&mut self, stars: i32) {
        self.stars = stars;
    }
}