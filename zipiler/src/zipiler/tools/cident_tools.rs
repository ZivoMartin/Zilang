use super::include::*;
use crate::zipiler::prog_manager::include::{STACK_REG, RDX_SIZE, ASM_SIZES};

pub struct CIdentTools {
    /// The mount of times that we have to dereferance our memory spot
    deref_time: i32,
    /// The name of our base variable
    name: String,
    /// If we catch an exp, its determine if its between a bracket or a tupple
    for_bracket: bool, 
    /// The number of expression before the potential equal sign, exemple for t[2][f(1)] whe have two
    nb_exp: u8,
    /// Working field used for save the address at the begining of the life of the tool.
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

    /// Very bad verification, we are saying if for_brackets is currently false then we are in a for_tupple handdling expression case so in function call.
    /// BUT if we have something like an array of functions pointers or a function who returns an array it not gonna work.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if !self.for_bracket { // We are catching a function call.
            self.raise_func_call(pm)
        }else{
            self.raise_mem_spot(pm)
        }
    }
}


impl CIdentTools {

    /// Set the equal code, exemple: "+=", "-=", "="
    fn set_equal_code(&mut self, equal_code: String) {
        self.equal_code = equal_code;
    }

    /// We can have two symbol, the star who increments the deref time and the ampersand who decrements the deref time. If we pass under -1, we are trying to
    /// dereference a reference so we throw an error. 
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

    /// Simply set the name of the entire thing
    fn def_ident(&mut self, name: String){
        self.name = name;
    }

    /// Called when we catch a bracket group. Its a very smart system, we are not pushing a group but a simple token its like flag indicates hey 
    /// the next expression are gonna be for a bracket usage.
    fn open_brackets(&mut self) -> Result<(), String>{
        self.nb_exp = 0;
        self.for_bracket = true;
        Ok(())
    }

    /// Called when we catch a tupple group. Its a very smart system, we are not pushing a group but a simple token its like flag indicates hey 
    /// the next expression are gonna be for a func call.
    fn open_tupple(&mut self, pm: &mut ProgManager) -> Result<(), String> {
        if self.deref_time == -1 {
            return Err(String::from("You can't get get the reference of a value."))
        }
        self.nb_exp = 0;
        if !pm.is_function(&self.name) {
            Err(format!("{} isn't a function", self.name))
        }else{
            self.for_bracket = false;
            Ok(())
        }
    }
    
    fn close(&mut self) {
        if self.for_bracket {

        }else{

        }
    }

    /// We catch a new expression, three cases, we already defined the equal code so this expression is the raising of the result of the right expression, so
    /// its on the stack and we have to do nothing. If the expression is for a bracket, same we juste increments the number of exp and let the result ont the stack.
    /// Else if its for a func call we have to check the validity of the type of the expression.
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

    // Raise the deref time, the number of stars of the entire memory spot and the size (4 if stars != 0)
    fn raise_mem_spot(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let var_def = pm.get_var_def_by_name(&self.name)?;
        let stars = var_def.type_var().stars() as i32 - self.deref_time - self.nb_exp as i32; // Here we compute the the global number of stars
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
        if pm.get_func_by_name(&self.name)?.return_type().name() != "void" {"push rax"}else{""});   // We just check if the function return something witch the name of the return type
        Ok((TokenType::FuncCall(pm.get_func_addr(&self.name)), asm))    

    }


    /// Take in parameter the number of stars of the entire memory spot, the mount of time we have to dereference it and the definition of the variable.    
    fn build_asm(&self, stars: i32, deref_time: i32, pm: &ProgManager, var_def: &VariableDefinition) -> String {
        let size = if stars == 0 {var_def.get_true_size()}else{POINTER_SIZE as u8};
        let mut res = if self.equal_code.is_empty() {String::new()}else{String::from("\npop rdi")};
        res.push_str(&format!("
mov rax, {}
add rax, {STACK_REG}", var_def.addr()));
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
pop rax     ; addr of the left ident
{}",         format!("{} {}[_stack + rax], {}", 
            match &self.equal_code as &str {
                "=" => "mov",
                "+=" => "add",
                "-=" => "sub",
                _ => panic!("Unknow equal code: {}", self.equal_code)
            },
            ASM_SIZES[size as usize], 
            RDX_SIZE[size as usize])))
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