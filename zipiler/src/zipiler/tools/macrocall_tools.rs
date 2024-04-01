use super::include::*;

/// Proke generaly when we detect a '!'
pub struct MacroCallTools {
    /// Name of the macro
    name: String,           
    /// nb_param: Number of parameter on the called macro
    nb_param: u8,           
    /// nb_param_attempt: Number of parameter awaited
    nb_param_attempt: u8   
}


/// The macro list with the number of parameter of each one
static MACRO_LIST: [(&str, u8); 3] = [("dn", 1), ("exit", 1), ("print_char", 1)];

static _SIZE_PARAM: u8 = 8;

impl Tool for MacroCallTools {

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        Box::from(MacroCallTools{
            name: String::new(),
            nb_param: 0,
            nb_param_attempt: 0
        })
    }


    /// If the number of parameter of the macro isn't equal to the stored number of argument we throw an error 
    /// otherwise we build asm code.
    fn end(&mut self, _pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if self.nb_param != self.nb_param_attempt {
            Err(format!("{} args has been found for the macro {} when {} was attempts", self.nb_param, self.name, self.nb_param_attempt))
        }else{
            let asm = self.build_asm();
            Ok((TokenType::MacroCall, asm))
        }
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        match token.token_type {
            TokenType::Ident => self.def_name(token.content)?,
            TokenType::RaiseExpression(_) => self.new_expression(),
            TokenType::ExpressionTuple => (),
            _ => pm.panic_bad_token("macro call", token)
        }
        Ok(String::new())
    }

}

impl MacroCallTools {


    /// Called when the name of the macro drops, gonna iterate through the macro list check if the macro exists,
    /// if it is store the name and the number of arguments, throw an error otherwise.
    fn def_name(&mut self, name: String) -> Result<(), String> {
        for m in MACRO_LIST.iter() {
            if m.0 == &name {
                self.name = m.0.to_string();
                self.nb_param_attempt = m.1;
                return Ok(())
            }
        }
        Err(format!("Unknow macro: {}", name))
    }

    /// Called when we detect an expression, this expression is anyway an arguments so we just increment the number
    /// of arg. The result of the expression is already on the stack
    fn new_expression(&mut self) {
        self.nb_param += 1;
    }

    /// We put the name of the macro, then each argument registered on the stack on one single line and return it
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