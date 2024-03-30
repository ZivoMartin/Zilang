use super::{include::*, prog_manager::ProgManager};



impl ProgManager {
    pub fn is_function(&self, name: &str) -> bool {
        self.func_name_map.contains_key(name) && !self.func_name_map.get(name).unwrap().is_empty()
    }

    pub fn new_function(&mut self, name: String, args: Vec<Type>, return_type: Type) {
        let f = Function::new(self.si(), name.clone(), args, return_type);
        match self.func_name_map.get_mut(&name) {
            Some(s) => s.push(self.stack_index),
            _ => {self.func_name_map.insert(name, Stack::init(self.si()));}
        };
        self.func_map.insert(self.si(), f);
        self.current_func = Some(self.si());
        // self.stack_index += 8;
    }

    pub fn get_func_addr(&self, name: &str) -> usize {
        self.get_func_by_name(name).unwrap().addr()
    }

    pub fn get_func_by_addr(&self, addr: usize) -> &Function {
        self.func_map.get(&addr).unwrap()
    }

    pub fn get_func_by_name(&self, name: &str) -> Result<&Function, String> {
        match self.func_name_map.get(name) {
            Some(s) => Ok(
                self.get_func_by_addr(*(
                    s.val().unwrap()))
                ), 
            _ => Err(format!("The function {name} doesn't exists."))
        }
    }

    pub fn new_var(&mut self, name_type: String, name: String, stars: u32) -> usize {
        let size = self.get_type_size_with_type_name(&name_type); 
        let var_def =  VariableDefinition::new(
            self.si(),
            name.clone(),
            Type::new(name_type, size, stars),
            self.stage
        );
        match self.var_map.get_mut(&self.si()) {
            Some(s) => s.push(var_def),
            _ => {self.var_map.insert(self.si(), Stack::init(var_def));}
        }
        if self.var_name_map.contains_key(&name) {
            self.var_name_map.get_mut(&name).unwrap().push(self.stack_index);
        }else{
            self.var_name_map.insert(
                name,
                Stack::init(self.si())
            );
        }
        let res = self.si();
        self.jump_stack.val_mut().expect("jump stack empty").add_addr(self.stack_index);
        self.stack_index += if stars == 0 { size as usize }else{POINTER_SIZE};
        res
    } 

    pub fn get_var_def_by_name(&self, name: &String) -> Result<&VariableDefinition, String> {
        let addr = match self.var_name_map.get(name) {
            Some(stack) => stack.val().expect("The stack of a var name is empty"),
            _ => return Err(format!("Variable {} doesn't exists", name)) 
        };
        let vd = self.get_var_def(addr)?;
        if vd.name() != name {
            Err(format!("The variable {} isn't richieble in this scope", name))
        }else{
            Ok(vd)
        }
    }
    
    pub fn get_var_def(&self, addr: &usize) -> Result<&VariableDefinition, String> {
        match self.var_map.get(addr) {
            Some(res) => { 
                let vd = res.val().expect("addr stack is empty");
                if vd.stage() != self.stage {
                    Err(format!("The variable {} isn't richieble in this scope", vd.name()))
                }else{
                    Ok(vd)
                }
            },
            _ => Err(String::from("Variable doesn't exists"))
        }
    }

    pub fn current_func(&self) -> &Function {
        if self.current_func.is_none() {
            panic!("You tried to get the address of the current function without being in a function")
        }
        self.get_func_by_addr(self.current_func.unwrap())
    }
}