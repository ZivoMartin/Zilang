use self::files::{FUNCTIONSF, SCRIPTF};

use super::collections::Stack;
use std::collections::HashMap;
use super::include::*;


pub static ASM_SIZES: [&str; 9] = ["", "byte", "word", "", "dword", "", "", "", "qword"];
pub static RAX_SIZE: [&str; 9] = ["", "al", "ax", "", "eax", "", "", "", "rax"];

pub struct Memory {
    var_name_map: HashMap<String, Stack<usize>>,
    var_map: HashMap<usize, VariableDefinition>,
    func_map: HashMap<String, Stack<Function>>,
    type_size: HashMap<String, u8>, 
    stack_index: usize,
    pub bloc_id: u128,
    pub if_count: u32,
    jump_stack: Stack<Jump>,
    pub current_file: usize
}

impl Memory {

    pub fn new() -> Memory {
        Memory {
            var_name_map: HashMap::new(),
            var_map: HashMap::new(),
            func_map: HashMap::new(),
            type_size: build_tab_size_map(),
            stack_index: 0,
            bloc_id: 0,
            if_count: 0,
            jump_stack: Stack::init(Jump::new(0)),
            current_file: SCRIPTF
        }
    }

    pub fn is_function(&self, name: &str) -> bool {
        self.func_map.contains_key(name) && !self.func_map.get(name).unwrap().is_empty()
    }

    pub fn new_function(&mut self, name: String, args: Vec<Type>, return_type: Type) {
        let f = Function{name: name.clone(), args, return_type, addr: self.stack_index};
        match self.func_map.get_mut(&name) {
            Some(s) => s.push(f),
            _ => {self.func_map.insert(name, Stack::init(f));}
        };
        self.stack_index += 8;
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
        self.jump_stack.val_mut().expect("jump stack empty").add_addr(self.stack_index);
        self.stack_index += size as usize;
        res
    } 

    pub fn get_var_def_by_name(&self, name: &String) -> Result<&VariableDefinition, ()> {
        let addr = match self.var_name_map.get(name) {
            Some(stack) => stack.val().expect("The stack of a var name is empty"),
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

    pub fn get_type_size(&self, nb_s: i32, name: &str) -> u8 {
        if nb_s != 0 {
            4
        }else{
            *self.type_size.get(name).expect("type doesn't exists")
        }
    }

    pub fn jump_in(&mut self) {
        self.jump_stack.push(Jump::new(self.stack_index));
    }

    pub fn jump_out(&mut self) {
        let last_jump = self.jump_stack.pop().expect("Can t jump out, stack empty");
        for addr in last_jump.addr_to_remove.iter() {
            let var_def = self.var_map.remove(addr).expect("Adress unvalid");
            self.var_name_map
                .get_mut(&var_def.name).expect("The name doesn't exists")
                .pop().expect("The varname stack is empty");
        }
        self.stack_index = last_jump.stack_index;
    }

    pub fn in_func(&mut self) {
        self.current_file = FUNCTIONSF;
    }

    pub fn out_func(&mut self) {
        self.current_file = SCRIPTF;
    }

    pub fn si(&self) -> usize {
        self.stack_index
    }

    pub fn handle_arg(&mut self, f_name: &str, stars: i32, nth: usize) -> Result<String, String> {
        let f = self.func_map.get_mut(f_name).unwrap().pop().unwrap();
        if f.args[nth as usize].stars != stars {
            return Err("Not the good type for the call.".to_string())
        }
        let size = self.get_type_size(stars, &f.args[nth as usize].name) as usize;
        let res = format!("\nmov {}[_stack + {}], {}", ASM_SIZES[size], self.stack_index, RAX_SIZE[size]);
        self.stack_index += size;
        Ok(res)
    }

    pub fn good_nb_arg(&mut self, name: &str, nb_arg: u8) -> Result<(), String>{
        if self.func_map.get_mut(name).unwrap().pop().unwrap().args.len() != nb_arg as usize {
            Err(String::from("not the good number of arg"))
        }else{
            Ok(())
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


