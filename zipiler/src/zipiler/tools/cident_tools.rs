use super::include::*;
use crate::zipiler::prog_manager::include::{STACK_REG, RAX_SIZE, ASM_SIZES};

pub struct CIdentTools {
    deref_time: i32,
    name: String,
    for_bracket: bool, // If we catch an exp, its determine if its between a bracket or a tupple
    nb_exp: u8,
    si_save: usize,
    /// Exemple: "+=", "-="...
    equal_code: String
}

impl Tool for CIdentTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
         match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content)?,
            TokenType::Ident => self.def_ident(token.content),
            TokenType::BrackTuple => self.close(),
            TokenType::Brackets => self.open_brackets()?,
            TokenType::ExpressionTuple => self.open_tupple(pm)?,
            TokenType::RaiseExpression(stars) => res = self.new_expression(pm, stars)?,
            TokenType::Operator => self.set_equal_code(token.content),
            _ => pm.panic_bad_token("complex ident", token)
        }
        Ok(res)
    }
    
    fn new(pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(CIdentTools {
            deref_time: 0,
            name: String::new(),
            for_bracket: true,
            nb_exp: 0,
            si_save: pm.si(),
            equal_code: String::new()
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

    fn set_equal_code(&mut self, equal_code: String) {
        self.equal_code = equal_code;
    }

    fn new_symbol(&mut self, s: String) -> Result<(), String>{
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

    fn def_ident(&mut self, name: String){
        self.name = name;
    }

    fn open_brackets(&mut self) -> Result<(), String>{
        self.nb_exp = 0;
        self.for_bracket = true;
        Ok(())
    }

    fn open_tupple(&mut self, pm: &mut ProgManager) -> Result<(), String> {
        if self.deref_time == -1 {
            return Err(String::from("You can't get get the reference of a value."))
        }
        self.nb_exp = 0;
        if !pm.is_function(&self.name) {
            Err(format!("{} isn't a function", self.name))
        }else{
            Ok(self.for_bracket = false)
        }
    }

    fn close(&mut self) {
        if self.for_bracket {

        }else{

        }
    }

    fn new_expression(&mut self, pm: &mut ProgManager, stars: i32) -> Result<String, String>{
        if self.equal_code.is_empty() {
            self.nb_exp += 1;
            if self.for_bracket {
                Ok(String::new( ))
            }else{
                pm.handle_arg(&self.name, stars, (self.nb_exp-1) as usize)
            }
        }else{
            Ok(String::new())
        }

    }

    // Raise the deref time, the number of stars and the size
    fn raise_mem_spot(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let var_def = pm.get_var_def_by_name(&self.name)?;
        let stars = var_def.type_var().stars() as i32 - self.deref_time - self.nb_exp as i32;
        if stars < -1 {
            return Err(format!("Bad dereferencment")) // If you want to modifie this line, care it could be dangerous because of the unsafe
        }
        let asm = self.build_asm(stars, self.deref_time, pm, var_def);
        Ok((TokenType::MemorySpot(self.deref_time, stars,   
            if stars == 0 {var_def.get_true_size()}else{POINTER_SIZE as u8}), asm))
    }

    // We raise the address of the function
    fn raise_func_call(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if self.deref_time == -1 {
            return Err(String::from("You can't get get the reference of a returned value."))
        }
        pm.good_nb_arg(&self.name, self.nb_exp)?;
        let asm = format!("
push {STACK_REG}
add {STACK_REG}, {}
call {}
pop {STACK_REG}

{}", self.si_save, self.name, 
        // pm.deref_var(pm.get_type_size(self.deref_time, &self.type_name) as usize, self.deref_time),
        if pm.get_func_by_name(&self.name)?.return_type().name() != "void" {"push rax"}else{""});
        Ok((TokenType::FuncCall(pm.get_func_addr(&self.name)), asm))

    }

    fn build_asm(&self, stars: i32, deref_time: i32, pm: &ProgManager, var_def: &VariableDefinition) -> String {
        let size = if stars == 0 {var_def.get_true_size()}else{POINTER_SIZE as u8};
        let mut res = format!("
mov rax, {}
add rax, {STACK_REG}", var_def.addr());
        for i in 0..self.nb_exp {
            res.push_str(&format!("
_deref_dword 1
mov r13, rax
mov rax, [rsp + {}]
{}
add rax, r13", (self.nb_exp-i-1)*8, mul_deref_string(i, self.nb_exp, var_def)))
        }
        res.push_str(&format!("
add rsp, {}", self.nb_exp*8));
        res.push_str(&pm.deref_var(var_def.type_var().size() as usize, deref_time));
        res.push_str("\npush rax    ; We push the value of a new identificator");
        if !self.equal_code.is_empty() {
            res.push_str(&format!("
pop rbx     ; addr of the left ident
pop rax     ; result of the expression
{}",         format!("{} {}[_stack + rbx], {}", 
            match &self.equal_code as &str {
                "=" => "mov",
                "+=" => "add",
                "-=" => "sub",
                _ => panic!("Unknow equal code: {}", self.equal_code)
            },
            ASM_SIZES[size as usize], 
            RAX_SIZE[size as usize])))
        }
        res
    }

}

fn mul_deref_string(i: u8, nb_exp: u8, var_def: &VariableDefinition) -> String {
    if i == nb_exp-1 {
        let s = var_def.get_true_size();
        if s == 1 {
            String::new()
        }else{
            format!("mov r12, {}\nmul r12", s)
        }
    } else {
        format!("mul {MUL_REGISTER}")
    }
}