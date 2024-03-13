use super::collections::Stack;
use std::collections::HashMap;
use super::include::*;


pub static ASM_SIZES: [&str; 9] = ["", "byte", "word", "", "dword", "", "", "", "qword"];
static RAX_SIZE: [&str; 9] = ["", "al", "ax", "", "eax", "", "", "", "rax"];

pub struct Memory {
    var_name_map: HashMap<String, Stack<usize>>,
    var_map: HashMap<usize, VariableDefinition>,
    //func_map: HashMap<usize, Function>,
    type_size: HashMap<String, u8>, 
    stack_index: usize,
    //nb_func: usize
}

impl Memory {

    pub fn new() -> Memory {
        Memory {
            var_name_map: HashMap::new(),
            var_map: HashMap::new(),
            //func_map: HashMap::new(),
            type_size: build_tab_size_map(),
            stack_index: 0,
            //nb_func: 0
        }
    }

    pub fn new_var(&mut self, name_type: String, name: String, stars: i32) -> usize {
        let size = *self.type_size.get(&name_type).unwrap(); 
        self.var_map.insert(
            self.stack_index,
            VariableDefinition{
                name: name.clone(),
                type_var: Type{
                    size,
                    name: name_type,
                    stars
                },
                addr: self.stack_index
            }
        );
        if self.var_name_map.contains_key(&name) {
            self.var_name_map.get_mut(&name).unwrap().push(self.stack_index);
        }else{
            self.var_name_map.insert(
                name,
                Stack::init(self.stack_index)
            );
        }
        let res = self.stack_index;
        self.stack_index += size as usize;
        res
    } 

    pub fn get_var_def_by_name(&self, name: &String) -> Result<&VariableDefinition, ()> {
        let addr = match self.var_name_map.get(name) {
            Some(stack) => stack.val(),
            _ => return Err(()) 
        };
        Ok(self.var_map.get(addr).unwrap())
    }
    
    pub fn get_var_def(&self, addr: &usize) -> Result<&VariableDefinition, ()> {
        match self.var_map.get(addr) {
            Some(res) => Ok(res),
            _ => Err(())
        }
    }

    pub fn affect_to(&self, addr: usize) -> String {
        let size = self.get_var_def(&addr).unwrap().type_var.size as usize;
        format!("\nmov {}[_stack + {}], {}", ASM_SIZES[size], addr, RAX_SIZE[size])
    }


    pub fn deref_var(&self, size: usize, stars: i32) -> String {
        if stars > 0 {
            format!("\n_deref_{} {}", ASM_SIZES[size], stars)
        }else{
            String::new()
        }
    }

}

fn build_tab_size_map() -> HashMap<String, u8> {
    let mut res = HashMap::new();
    res.insert(String::from("int"), 4);
    res.insert(String::from("char"), 1);
    res.insert(String::from("void"), 0);
    res
}


