use crate::zipiler::tools::include::*;

pub struct NewKeyword {
    /// Name of the class object we are building
    class_name: String,
    /// Number of expression detected
    nb_exp: usize,
    /// Stack index before the allocation of the args for the contstructor
    save_si: usize
}

impl Tool for NewKeyword {


    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
        match token.token_type {
            TokenType::RaiseExpression(nb_stars) => res = self.new_exp(pm, nb_stars)?,
            TokenType::Ident => res = self.set_class_name(pm, token.content)?,
            _ => pm.panic_bad_token("new keyword", token)
        }
        Ok(res)
    }


    fn new(pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(NewKeyword{
            class_name: String::new(),
            nb_exp: 0,
            save_si: pm.si()
        })
    }


    /// We raise the id of the class we just allocate. The address of the mem spot is on the stack.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm(pm)?;
        Ok((TokenType::RaiseNewClass(pm.get_class_by_name(&self.class_name).id()), asm))
    }
}

impl NewKeyword {

    fn set_class_name(&mut self, pm: &mut ProgManager, name: String) -> Result<String, String>{
        if !pm.class_exists(&name) {
            return Err(format!("The class {} doesn't exists.", name))
        }
        self.class_name = name;
        let heap_addr = pm.allocate_new_object(&self.class_name);
        let asm = format!("
push {ha}
mov dword[_stack + {STACK_REG} + {}], {ha}", pm.si(), ha=heap_addr);

        Ok(asm)
    }

    fn new_exp(&mut self, pm: &mut ProgManager, nb_star: i32) -> Result<String, String>{
        let constructor = pm.get_class_by_name(&self.class_name).get_constructor().clone();
        self.nb_exp += 1;
        pm.handle_arg(&constructor, nb_star, self.nb_exp-1)
    }

    fn build_asm(&self, pm: &mut ProgManager) -> Result<String, String> {
        pm.stack_index += POINTER_SIZE;     // For self 
        let constructor = pm.get_class_by_name(&self.class_name).get_constructor();
        constructor.good_nb_arg(self.nb_exp as u8)?;
        let asm = format!("
push {STACK_REG}
add {STACK_REG}, {}
mov rax, [_progmem + {}]
call rax
pop {STACK_REG}", self.save_si, constructor.addr());
        pm.stack_index = self.save_si;
        Ok(asm)
    }

}