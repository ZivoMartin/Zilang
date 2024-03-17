
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


pub struct Type {
    pub name: String,
    pub size: u8,
    pub stars: i32
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


#[allow(dead_code)]
pub struct Function {
    pub args: Vec::<VariableDefinition>,
    pub return_type: Type,
    name: String
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

