pub use crate::hammer::collections::Stack;
pub use std::collections::HashMap;
pub use self::files::{FUNCTIONSF, SCRIPTF};

pub static ASM_SIZES: [&str; 9] = ["", "byte", "word", "", "dword", "", "", "", "qword"];
pub static RAX_SIZE: [&str; 9] = ["", "al", "ax", "", "eax", "", "", "", "rax"];


pub struct Jump {
    pub stack_index: usize,
    pub addr_to_remove: Vec<usize>
}

impl Jump {

    pub fn new(stack_index: usize) -> Jump {
        Jump{stack_index, addr_to_remove: Vec::new()}
    }

    pub fn add_addr(&mut self, addr: usize) {
        self.addr_to_remove.push(addr);
    }

}

pub struct Function {
    addr: usize,
    name: String,
    args: Vec<Type>,
    return_type: Type
}

#[allow(dead_code)]
impl Function {

    pub fn new(addr: usize, name: String, args: Vec<Type>, return_type: Type) -> Function {
        Function{addr, name, args, return_type}
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn args(&self) -> &Vec<Type> {
        &self.args
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }


    pub fn nb_arg(&self) -> usize {
        self.args().len()
    }
}


#[derive(Debug)]
pub struct Type {
    name: String,
    size: u8,
    stars: u32
}

impl std::hash::Hash for Type{
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher{
        self.name.hash(state);
        self.size.hash(state);
        self.stars.hash(state);
    }
}
 
impl Eq for Type{}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        self.stars == other.stars && self.size == other.size && self.name == other.name
    }
}

impl Clone for Type {
    fn clone(&self) -> Type {
        Type{
            name: self.name.clone(),
            size: self.size,
            stars: self.stars
        }
    }
}

impl Type {
    pub fn new(name: String, size: u8, stars: u32) -> Type {
        Type{name, size, stars}
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn stars(&self) -> u32 {
        self.stars
    }
}

pub struct VariableDefinition{
    pub name: String,
    pub type_var: Type,
    pub addr: usize
}

impl VariableDefinition {

    pub fn get_size(&self) -> u8{
        if self.type_var.stars == 0 {
            self.type_var.size
        }else{
            4
        }
    }

}

pub static F_PATHS: [&str; 6] = [
                "asm/script.asm",
                "asm/base_files/base_script.asm",
                "asm/functions.asm",
                "asm/base_files/base_functions.asm",
                "asm/base_files/base_data.asm",
                "asm/base_files/base_macros.asm"
                ]; 

#[allow(dead_code)]
pub mod files {
    pub static SCRIPTF: usize = 0;
    pub static BASE_SCRIPTF: usize = 1;
    pub static FUNCTIONSF: usize = 2;
    pub static BASEFUNCTIONSF: usize = 3;
    pub static DATAF: usize = 4;
    pub static MACROSF: usize = 5;    
}

