use crate::tools::collections::Stack;
use std::collections::HashMap;
use super::include::*;

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

    pub fn new_var(&mut self, name_type: String, name: String, stars: i32) {
        let size = *self.type_size.get(&name_type).unwrap(); 
        self.var_map.insert(
            self.stack_index,
            VariableDefinition{
                name: name.clone(),
                type_var: Type{
                    size,
                    name: name_type,
                    stars
                }
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
        self.stack_index += size as usize;
    } 

    pub fn get_var_def(&self, name: &String) -> Result<&VariableDefinition, ()> {
        let addr = match self.var_name_map.get(name) {
            Some(stack) => stack.val(),
            _ => return Err(()) 
        };
        Ok(self.var_map.get(addr).unwrap())
    }

}

fn build_tab_size_map() -> HashMap<String, u8> {
    let mut res = HashMap::new();
    res.insert(String::from("int"), 4);
    res.insert(String::from("char"), 1);
    res.insert(String::from("void"), 0);
    res
}


