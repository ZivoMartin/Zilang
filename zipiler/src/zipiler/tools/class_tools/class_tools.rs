use crate::zipiler::tools::include::*;
pub struct ClassTools {
    name: String
}

impl Tool for ClassTools {


    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::RaiseDeclaration(addr) => self.new_attr(pm, addr),
            TokenType::Ident => self.set_class_name(pm, token.content),
            TokenType::RaiseFuncDef(addr) => self.new_method(pm, addr),
            _ => pm.panic_bad_token("class", token)
        }
        Ok(String::new())
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(ClassTools{
            name: String::new()
        })
    }

    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        pm.current_class = String::new();
        Ok((TokenType::ClassDefinition, String::new()))
    }
}

impl ClassTools {

    /// Set the name of the entire class we are defining
    fn set_class_name(&mut self, pm: &mut ProgManager, name: String) {
        self.name = name;
        pm.add_class(self.name.clone())
    }

    fn new_attr(&mut self, pm: &mut ProgManager, addr: usize) {
        let var_name = pm.get_var_def(&addr).expect("Failed to store attribute in memory").name().clone();
        let var_type = pm.get_var_def(&addr).unwrap().type_var().clone();
        pm.get_class_by_name_mut(&self.name).add_attr(var_name, var_type);    
    }

    fn new_method(&mut self, pm: &mut ProgManager, addr: usize) {
        let func_def = pm.get_func_by_addr(addr).clone();
        pm.get_class_by_name_mut(&self.name).add_meth(func_def)
    }
}