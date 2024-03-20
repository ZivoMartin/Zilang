use super::include::*;

pub struct CIdentTools {
    deref_time: i32,
    name: String,
    for_bracket: bool, // If we catch an exp, its determine if its between a bracket or a tupple
    nb_exp: u8
}

impl Tool for CIdentTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
         match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content)?,
            TokenType::Ident => self.def_ident(token.content),
            TokenType::Brackets => self.open_brackets(),
            TokenType::ExpressionTuple => self.open_tupple(pm)?,
            TokenType::Expression => res = self.new_expression(pm, token.content)?,
            _ => panic_bad_token("complex ident", token)
        }
        Ok(res)
    }
    
    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(CIdentTools {
            deref_time: 0,
            name: String::new(),
            for_bracket: true,
            nb_exp: 0
        })
    }

    /// We raise at first the deref_time, if its equals to -1 if we are looking for a direct reference, 
    /// otherwise 0 if we just want the value of the ident or x which is the value but dereferenced x times.
    /// We raise at second the number of stars of the ident, if t is of type
    /// int*, t has 1 star and t[2] has 0, *t[2] has -1 so its invalid
    /// We raise at third the size of the type, 4 for a pointer and the type size otherwise.
    /// In asm we are gonna push on the stack the reference of the value we are looking for, exemple if 'a' has address 3 and 
    /// the value 8, we are gonna push 3, then if we want the value of a
    /// we keep the adress on the stack and keep the value in the memory.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if !self.for_bracket { // We are catching a function call.
            self.raise_func_call(pm)
        }else{
            self.raise_mem_spot(pm)
        }
    }
}


impl CIdentTools {


    pub fn new_symbol(&mut self, s: String) -> Result<(), String>{
        if s == "*" {
            self.deref_time += 1;
        }else if s == "&" {
            self.deref_time -= 1;
            if self.deref_time < -1 {
                return Err(String::from("You can't dereferance like this.."))
            }
        }else{
            panic!("Bad symbol for a complexident: {s}")
        }
        Ok(())
    }

    pub fn def_ident(&mut self, name: String){
        self.name = name;
    }

    pub fn open_brackets(&mut self) {
        self.nb_exp = 0;
        self.for_bracket = true;
    }

    pub fn open_tupple(&mut self, pm: &mut ProgManager) -> Result<(), String> {
        self.nb_exp = 0;
        if !pm.is_function(&self.name) {
            Err(format!("{} isn't a function", self.name))
        }else{
            Ok(self.for_bracket = false)
        }
    }

    pub fn new_expression(&mut self, pm: &mut ProgManager, stars: String) -> Result<String, String>{
        self.nb_exp += 1;
        if self.for_bracket {
            todo!("New expression for bracket");
        }else{
            pm.handle_arg(&self.name, stars.parse::<i32>().unwrap(), (self.nb_exp-1) as usize)
        }

    }

    fn raise_mem_spot(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let var_def = match pm.get_var_def_by_name(&self.name) {
            Ok(var_def) => var_def,
            Err(_) => return Err(format!("{} isn't an axisting variable.", &self.name))
        };
        let stars = var_def.type_var.stars() as i32 - self.deref_time;
        if stars < -1 {
            return Err(format!("Bad dereferencment")) // If you want to modifie this line, care it could be dangerous because of the unsafe
        }

        let asm = self.build_asm(stars, self.deref_time, pm, var_def);
        Ok((TokenType::MemorySpot(self.deref_time, stars, var_def.get_size()), asm))
    }

    fn raise_func_call(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        pm.good_nb_arg(&self.name, self.nb_exp)?;
        let asm = format!("
call {}
; Todo: dereferanecer le retour (eventuellement)
push rax", self.name);
        Ok((TokenType::FuncCall(pm.get_func_addr(&self.name)), asm))

    }

    fn build_asm(&self, _stars: i32, deref_time: i32, pm: &ProgManager, var_def: &VariableDefinition) -> String {
        let mut res = format!("\nmov rax, {}", var_def.addr);
        res.push_str(&pm.deref_var(var_def.type_var.size() as usize, deref_time));
        res.push_str("\npush rax    ; We push the value of a new identificator");
        res
    }

}