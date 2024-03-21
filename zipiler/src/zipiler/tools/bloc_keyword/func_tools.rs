use crate::zipiler::tools::include::*;

pub struct FuncTools {
    name: String,
    type_args: Vec<Type>,
    return_type: Type
}

impl Tool for FuncTools {

    fn new(pm: &mut ProgManager) -> Box<dyn Tool> where Self: Sized {
        pm.in_func();
        Box::from(
            FuncTools{
                name: String::new(),
                type_args: Vec::new(),
                return_type: Type::new(String::new(), 0, 0)
            }
        )
    }

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String> {
        let mut res = String::new();
        match token.token_type {
            TokenType::RaiseDeclaration(addr) => self.new_arg(pm, addr),
            TokenType::Ident => res = self.set_ident(token.content),
            TokenType::RaiseComplexType(id, stars, size) => self.set_type(pm, id, stars as u32, size),
            TokenType::Bloc => res = self.end_of_func(),
            _ => panic_bad_token("func keyword", token)
        }
        Ok(res)
    }

    
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        pm.out_func();
        Ok((TokenType::FuncKeyword, String::new()))
    }
}

impl FuncTools {

    fn new_arg(&mut self, pm: &mut ProgManager, addr: usize) {
        let var_def = pm.get_var_def(&addr).unwrap();
        self.type_args.push(var_def.type_var.clone())
    }

    fn set_ident(&mut self, name: String) -> String {
        self.name = name;
        format!("{}:", self.name)
    } 

    fn set_type(&mut self, pm: &mut ProgManager, id: usize, stars: u32, size: u8) {
        let name = pm.get_type_name_with_id(id);
        self.return_type = Type::new(name, size, stars);
        pm.preload(format!("
mov qword[_stack + {}], {}", pm.si(), self.name
        ));
        pm.new_function(self.name.clone(), self.type_args.clone(), self.return_type.clone());
    }

    fn end_of_func(&self) -> String {
        String::from("\nret")
    }

}