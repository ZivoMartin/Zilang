pub use crate::zipiler::collections::Stack;
pub use std::collections::HashMap;
pub use crate::zipiler::tokenizer::include::{Token, TYPE_LIST};

pub static ASM_SIZES: [&str; 9] = ["", "byte", "word", "", "dword", "", "", "", "qword"];
pub static RAX_SIZE: [&str; 9] = ["", "al", "ax", "", "eax", "", "", "", "rax"];
pub static RDX_SIZE: [&str; 9] = ["", "dil", "di", "", "edi", "", "", "", "rdx"];

pub static STACK_REG: &str = "r15";
pub static MUL_REGISTER: &str = "r14";
pub static POINTER_SIZE: usize = 4; 
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

impl Clone for Function {
    fn clone(&self) -> Function {
        Function{
            addr: self.addr(), 
            name: self.name().clone(), 
            args: self.args().clone(), 
            return_type: self.return_type().clone()
        }
    }
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

    pub fn check_valid_return_type(&self, stars: i32) -> bool {
        if stars == -1 {
            self.return_type.name() == "void"
        }else{
            self.return_type.stars() == stars as u32
        }
    }
}


#[derive(Debug)]
pub struct Type {
    name: String,
    size: u8,
    stars: u32,
    class: Option<usize>
}

impl std::hash::Hash for Type{
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher{
        self.name.hash(state);
        self.size.hash(state);
        self.stars.hash(state);
        self.class.hash(state);
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
            stars: self.stars,
            class: self.class.clone()
        }
    }
}

impl Type {
    pub fn new(name: String, size: u8, stars: u32, class: Option<usize>) -> Type {
        Type{name, size, stars, class}
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> u8 {
        if self.class.is_some() {
            return POINTER_SIZE as u8;
        }
        self.size
    }

    pub fn stars(&self) -> u32 {
        self.stars
    }
    
    pub fn get_class(&self) -> Option<usize> {
        self.class
    }
}

pub struct VariableDefinition{
    name: String,
    type_var: Type,
    addr: usize,
    stage: u32
}

impl VariableDefinition {

    pub fn new(addr: usize, name: String, type_var: Type, stage: u32) -> VariableDefinition {
        VariableDefinition{name, addr, type_var, stage}
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn type_var(&self) -> &Type {
        &self.type_var
    }

    pub fn stage(&self) -> u32 {
        self.stage
    }

    pub fn addr(&self) -> usize {
        self.addr
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> u8{
        if self.type_var.stars == 0 {
            self.type_var.size
        }else{
            4
        }
    }

    pub fn get_true_size(&self) -> u8 {
        self.type_var.size
    }

}

pub static F_PATHS: [&str; 4] = [
                "asm/script.asm",
                "asm/base_files/base_script.asm",
                "asm/base_files/base_data.asm",
                "asm/base_files/base_macros.asm"
                ]; 

#[allow(dead_code)]
pub mod files {
    pub static SCRIPTF: usize = 0;
    pub static BASE_SCRIPTF: usize = 1;
    pub static MACROSF: usize = 2;    
}
#[allow(dead_code)]
pub struct Class {
    name: String,
    attributes: HashMap<String, Type>,
    methods: HashMap<String, Function>,
    size: u8,
    id: usize
}

#[allow(dead_code)]
impl Class {

    pub fn new(name: String, id: usize) -> Class {
        Class{name, id, methods: HashMap::new(), attributes: HashMap::new(), size: 0}
    }

    pub fn clone(&self) -> Class {
        Class {
            name: self.name.clone(),
            methods: self.methods.clone(),
            size: self.size,
            attributes: self.attributes.clone(),
            id: self.id
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn attribute_exists(&self, name: &String) -> bool {
        return self.attributes.contains_key(name)
    }

    pub fn method_exists(&self, name: &String) -> bool {
        return self.methods.contains_key(name)
    }
    pub fn get_attribute_type(&self, name: &String) -> Result<&Type, String> {
        if !self.attribute_exists(name) {
            Err(format!("Attribute {} doesn't exists", name))
        }else{
            Ok(self.attributes.get(name).unwrap())
        }
    }

    pub fn add_attr(&mut self, name: String, type_attr: Type) {
        self.size += type_attr.size();
        self.attributes.insert(name, type_attr);
    }

    pub fn add_meth(&mut self, f: Function) {
        self.methods.insert(f.name().clone(), f);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}