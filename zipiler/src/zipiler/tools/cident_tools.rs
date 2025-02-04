use super::include::*;

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
    equal_code: String,
    field: String,
    class: Option<usize>
}

impl Tool for CIdentTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
         match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content)?,
            TokenType::Ident => self.new_ident(pm,token.content)?,
            TokenType::BrackTuple => self.close(),
            TokenType::Brackets => self.open_brackets()?,
            TokenType::ExpressionTuple => res = self.open_tupple(pm)?,
            TokenType::RaiseExpression(stars) => res = self.new_expression(pm, stars)?,
            TokenType::Operator => self.set_equal_code(token.content),
            TokenType::RaiseNewClass(_) => res = self.new_expression(pm, 0)?,
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
            equal_code: String::new(),
            field: String::new(),
            class: None
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
    fn new_ident(&mut self, pm: &ProgManager, name: String) -> Result<(), String>{
        if self.name.is_empty() {
            self.name = name;
        }else{
            self.deref_time += 1;
            let type_var = if self.for_bracket {
                pm.get_var_def_by_name(&self.name)?.type_var()
            } else {
                pm.get_func_by_name(&self.name)?.return_type()
            };
            if let Some(class_id) = type_var.get_class() {
                let class = pm.get_class(class_id);
                if !(class.attribute_exists(&name) || class.method_exists(&name)) {
                    return Err(format!("The class {} doesn't have a field named {}", class.get_name(), name))
                }
                self.class = Some(class.id());
                self.field = name;
            } else {
                return Err(format!("The variable {} is not an object.", self.name))
            }
        }
        Ok(())
    }

    /// Called when we catch a bracket group. Its a very smart system, we are not pushing a group but a simple token as a flag indicates hey 
    /// the next expression are gonna be for a bracket usage.
    fn open_brackets(&mut self) -> Result<(), String>{
        self.nb_exp = 0;
        self.for_bracket = true;
        Ok(())
    }

    /// Called when we catch a tupple group. Its a very smart system, we are not pushing a group but a simple token its like flag indicates hey 
    /// the next expression are gonna be for a func call.
    fn open_tupple(&mut self, pm: &mut ProgManager) -> Result<String, String> {
        if self.deref_time == -1 {
            return Err(String::from("You can't get get the reference of a value."))
        }
        let mut res = String::new();
        self.nb_exp = 0;
        if !pm.is_function(&self.name) {
            let var_def = pm.get_var_def_by_name(&self.name)?;
            if var_def.type_var().stars() - self.deref_time as u32 != 0 {
                return Err(format!("You tried to access to a field of a pointer.."))
            }
            if let Some(id) = var_def.type_var().get_class() {
                let class = pm.get_class(id);
                if !class.method_exists(&self.field) {
                    return Err(format!("{} is an attribute of the class {}, not a method.", self.field, class.get_name()))
                }
                res = format!("
mov eax, dword[_stack+{STACK_REG}+{}]
mov dword[_stack+{STACK_REG}+{}], eax", var_def.addr(), pm.si());
                pm.stack_index += POINTER_SIZE;
            }else {
                return Err(format!("{} isn't a function", self.name))
            }
        }
        self.for_bracket = false;
        Ok(res)
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
                let f = if self.field.is_empty() { pm.get_func_by_name(&self.name)?.clone() } else { pm.get_var_def_by_name(&self.name)?.get_class(pm).unwrap().get_method(&self.field).clone() };
                pm.handle_arg(&f, stars, (self.nb_exp-1) as usize)
            }
        }else{
            Ok(String::new())
        }

    }

    // Raise the deref time, the number of stars of the entire memory spot and the size (4 if stars != 0) and the memSpot
    fn raise_mem_spot(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let var_def = pm.get_var_def_by_name(&self.name)?;
        let stars = var_def.type_var().stars() as i32 - self.deref_time - self.nb_exp as i32; // Here we compute the the global number of stars
        if stars < -1 {
            return Err(format!("Bad dereferencment")) // If you want to modifie this line, care it could be dangerous because of the unsafe
        }
        let asm = self.build_asm(stars, self.deref_time, pm, var_def);
        Ok((TokenType::MemorySpot(self.deref_time, stars,   
            if stars == 0 {var_def.get_true_size()}else{POINTER_SIZE as u8},
            if self.field.is_empty(){ MemZone::Stack }else{ MemZone::Heap }), asm))
    }

    // We raise the address of the function
    fn raise_func_call(&self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        if self.deref_time == -1 {
            return Err(String::from("You can't get get the reference of a returned value."))
        }
        let f = if self.field.is_empty() { pm.get_func_by_name(&self.name)? } else { pm.get_var_def_by_name(&self.name)?.get_class(pm).unwrap().get_method(&self.field) };
        f.good_nb_arg(self.nb_exp)?;
        let asm = format!("
push {STACK_REG}
add {STACK_REG}, {}
mov rax, [_progmem + {}]
call rax
pop {STACK_REG}

{}", self.si_save,  f.addr(), 
        // pm.deref_var(pm.get_type_size(self.deref_time, &self.type_name) as usize, self.deref_time),
        if f.return_type().name() != "void" {"push rax"}else{""});   // We just check if the function return something witch the name of the return type
        let f_addr = f.addr();
        pm.stack_index = self.si_save;
        Ok((TokenType::FuncCall(f_addr), asm))    
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
mov eax, dword[_stack + eax]
mov r13, rax
mov rax, [rsp + {}]
{}
add rax, r13", (self.nb_exp-i-1)*8, mul_deref_string(i, self.nb_exp, var_def)))
        }
        if self.nb_exp != 0 {
            res.push_str(&format!("\nadd rsp, {}", self.nb_exp*8));
        }
        res.push_str(&pm.deref_var(var_def.type_var().size() as usize, deref_time, MemZone::Stack));
        self.handle_field(pm, &mut res);
        if !self.equal_code.is_empty() {
            res.push_str(&format!("
{} {}[{} + rax], {}", 
            match &self.equal_code as &str {
                "=" => "mov",
                "+=" => "add",
                "-=" => "sub",
                _ => panic!("Unknow equal code: {}", self.equal_code)
            },
            ASM_SIZES[size as usize], 
            if self.field.is_empty() {"_stack"} else {"_heap"},
            RDX_SIZE[size as usize]));
        }else{
            res.push_str("\npush rax");
        }
        res
    }


    fn handle_field(&self, pm: &ProgManager, asm: &mut String) {
        if let Some(id) = self.class {
            let class = pm.get_class(id);
            asm.push_str(&format!("\nadd rax, {}", class.get_field_decall(&self.field)))
        }
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
