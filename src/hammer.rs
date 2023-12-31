#[allow(dead_code)]
pub mod hammer{
    use std::collections::HashMap;
    use crate::stack::Stack;
    use crate::tools::tools::{Tools, split, is_value, count_occur};

    struct Type {
        name: String,
        id: i32,
        size: i32
    }

    impl Type{
        pub fn clone(&self) -> Type{
            Type{
                name: self.name.clone(),
                id: self.id,
                size: self.size
            }
        } 
    }

    struct VariableDefinition{
        name: String,
        type_var: Type,
    }
    
    impl VariableDefinition{

        pub fn clone(&self) -> VariableDefinition{
            VariableDefinition{
                name: self.name.clone(),
                type_var: self.type_var.clone(),
            }
        }

    }

    struct Variable<T>{
        addr: i32,
        value: T
    }

    impl<T: Copy> Variable<T>{

        pub fn new(addr: i32, value: T) -> Variable<T>{
            Variable{
                addr: addr,
                value: value
            }
        }

        pub fn clone(&self) -> Variable<T>{
            Variable{
                addr: self.addr,
                value: self.value,
            }
        }
    }

    struct Function {
        name: String,
        type_return: Type,
        args: Vec::<VariableDefinition>
    }

    impl Function{

        pub fn clone(&self) -> Function{
            Function{
                name: self.name.clone(),
                type_return: self.type_return.clone(),
                args: self.clone_arg_tab()
            }
        }
        
        pub fn clone_arg_tab(&self) -> Vec::<VariableDefinition>{
            let mut result = Vec::<VariableDefinition>::new();
            for i in 0..self.args.len(){
                result.push(self.args[i].clone());
            }
            return result;
        }
    }

    struct Instruction {
        function: Function,
        args: Vec::<i64>
    }

    struct Memory {
        base_type: (HashMap<i32, Variable<i32>>, HashMap<i32, Variable<i32>>),
    }
    
    impl Memory {
        pub fn new() -> Memory {
            Memory {
                base_type: (HashMap::new(), HashMap::new()),
            }
        }
    }
    

    struct Hammer<'a>{
        tools: Tools,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<i32>>,
        addr_list: HashMap::<i32, VariableDefinition>,
        func_list: HashMap::<String, Function>,
        inst_list: Vec::<Instruction>,
        memory: Memory,
        jumps_stack: Stack<Vec::<i32>>,
        authorized_char_for_variable: &'a str
    }

    impl<'a> Hammer<'a>{
        pub fn new()->Hammer<'a>{
            let mut res = Hammer{
                tools: Tools::new(),
                type_list: HashMap::<String, Type>::new(),
                defined_var_list: HashMap::<String, Stack::<i32>>::new(),
                addr_list: HashMap::<i32, VariableDefinition>::new(),
                func_list: HashMap::<String, Function>::new(),
                inst_list: Vec::new(),
                memory: Memory::new(),
                jumps_stack: Stack::new(),
                authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_"
            };
            res.init_dispo_type();
            res.init_dispo_func();
            return res
        }

        pub fn jump_out(&mut self){
            let variable_to_destroy: Vec::<i32> = self.jumps_stack.pop();
            for addr in variable_to_destroy{
                let def = self.addr_list.remove(&addr).unwrap();
                self.defined_var_list.get_mut(&def.name).unwrap().pop();
                match def.type_var.id {
                    0 => {self.memory.base_type.0.remove(&addr);},
                    1 => {self.memory.base_type.1.remove(&addr);},
                    _ => panic!("Impossible jump")
                }
            }
        }

        fn init_dispo_func(&mut self){
            self.func_list.insert(
                String::from("DISPLAY"), 
                Function{
                    name: String::from("DISPLAY"), 
                    type_return: self.type_list["VOID"].clone(), 
                    args: vec!{
                        VariableDefinition{
                            name: String::from("x"),
                            type_var: self.type_list["INT"].clone(),
                        }
                    }
                }
            );
            self.func_list.insert(
                String::from("EXIT"), 
                Function{
                    name: String::from("EXIT"), 
                    type_return: self.type_list["VOID"].clone(), 
                    args: vec!{
                        VariableDefinition{
                            name: String::from("code"),
                            type_var: self.type_list["INT"].clone(),
                        }
                    }
                }
            );
           
        }

        fn init_dispo_type(&mut self){
            self.new_type(String::from("INT"), 4);
            self.new_type(String::from("INT*"), 4);
            self.new_type(String::from("VOID"), 0);
        } 

        fn new_type(&mut self, name: String, size: i32){
            self.type_list.insert( name.clone(), Type{name: name, id: self.type_list.len() as i32, size: size});   
        }

        pub fn is_valid_name(&self, name: &str) -> bool{
            for letter in name.chars(){
                if !self.authorized_char_for_variable.contains(letter){
                    return false;
                }
            }
            true
        }



        pub fn type_exists(&self, name: &str) -> bool{
            self.type_list.contains_key(name)
        }

        pub fn var_exists(&self, name: &str) -> bool{
            self.defined_var_list.contains_key(name)
        }

        pub fn func_exists(&self, name: &str) -> bool{
            self.func_list.contains_key(name)
        }

        pub fn define_new_var(&mut self, name: String, type_var: String) -> Result<(), String>{
            if !self.type_exists(&type_var){
                return Err(format!("{} is not a valid type.", type_var));
            }
            if !self.is_valid_name(&name){
                return Err(format!("{} is not a valid name for a variable.", name));
            }
            let addr = self.defined_var_list.len() as i32;
            
            if !self.var_exists(&name){
                let mut new_stack = Stack::<i32>::new();
                new_stack.push(addr);
                self.defined_var_list.insert(
                    name.clone(),
                    new_stack
                );    
            }else{
                self.defined_var_list.get_mut(&name).unwrap().push(addr);
            }
            
            self.addr_list.insert(
                addr,
                VariableDefinition{
                    name: name.clone(),
                    type_var: self.type_list[&type_var].clone()
                }
            );

            match &type_var as &str{
                "INT" => {self.memory.base_type.0.insert(addr, Variable::new(addr, 0));}
                _ => return Err(String::from("Impossible error"))
            }
            Ok(())
        }

        pub fn is_valid_parameter(&self, func: &Function, param: Vec::<String>, line_number: usize) -> Result<(), String>{
            if func.args.len() != param.len(){
                if func.args.len() > param.len(){
                    return Err(format!("Line {}: Not enough parameter for the function {}.", line_number, func.name));
                }else{
                    return Err(format!("Line {}: To many parameters for the function {}.", line_number, func.name));
                }
            }
            let mut gen_type = String::new();
            for i in 0..param.len(){
                if !self.var_exists(&param[i]){
                    return Err(format!("Line {}: No variable named {}", line_number, param[i]));
                }
                let await_type = func.args[i].type_var.name.clone();
                let var_addr = self.defined_var_list[&param[i]].val();
                let mut param_type = self.addr_list[&var_addr].type_var.name.clone();
                if await_type != param_type{
                    if await_type == "GEN"{
                        if gen_type == String::from(""){
                            gen_type = param_type;
                        }else if gen_type != param_type{
                            return Err(format!("Line {}: The variable {} has the type {} when {} was await", line_number, param[i], param_type, gen_type));
                        }
                    }else if await_type == "GEN*"{
                        if !param_type.ends_with("*"){
                            return Err(format!("Line {}: The variable {} has the type {} when a generic pointer was await", line_number, param[i], param_type));
                        }else if gen_type == String::from(""){
                            gen_type = param_type;
                            gen_type.pop();
                        }else{
                            param_type.pop();
                            if param_type != gen_type{
                                return Err(format!("Line {}: The variable {} has the type {} when {} was await", line_number, param[i], param_type, gen_type));
                            }
                        }
                    }else{
                        return Err(format!("Line {}: The variable {} has the type {} when {} was await", line_number, param[i], param_type, await_type));
                    }
                }
            }
            Ok(())
        }

        pub fn try_to_affect(&mut self, addr: i32, new_val_def: VariableDefinition, string_value: String, line_number: usize) -> Result<bool, String>{
            let var_def = self.addr_list[&addr].clone();
            if var_def.type_var.name != new_val_def.type_var.name{
                return Err(format!("Line {}: {} and {} doesn't have the same type.", line_number, var_def.name, new_val_def.name));
            }
            let value: i64 = string_value.parse::<i64>().unwrap();
            match var_def.type_var.id{
                0 => {self.memory.base_type.0.get_mut(&addr).unwrap().value = value as i32}
                1 => {self.memory.base_type.1.get_mut(&addr).unwrap().value = value as i32}
                _ => panic!("Impossible type")
            } 
            Ok(true)
        }

    }

    pub fn compile_txt(input: String) -> Result<String, String>{
        let mut hammer: Hammer = Hammer::new();
        let mut vec = split(&input, ";");
        let mut line_number = begin_loop(&mut vec)?;
        if line_number.1 == vec.len() {
            return Ok(get_assembly_txt(hammer)?);
        }else if vec[line_number.1].contains("INIT") {
            line_number = init_loop(&mut hammer, line_number, &mut vec)?;
        }
        if line_number.1 != vec.len(){
            instruct_loop(&mut hammer, line_number, &mut vec)?;
        }
        Ok(get_assembly_txt(hammer)?)
    }

    fn begin_loop(vec: &mut Vec::<String>) -> Result<(usize, usize), String>{
        let mut line_number = (1, 0);
        loop{
            if line_number.1 == vec.len(){
                break;
            }
            let mut line = split(&vec[line_number.1], " ");
            line_number.0 += clean_line(&mut line);
            match line.len(){
                1 => {
                    if line[0] == "INIT" || line[0] == "TEXT"{
                        break;
                    }else if line[0] != "MAIN"{
                        return Err(format!("Line {}: Syntax error.", line_number.0));
                    }
                }
                _ => {
                    if line.len() != 0  {
                        return Err(format!("Line {}: syntax error.", line_number.0));
                    }
                }
            }
            line_number.1 += 1;
        }
        Ok(line_number)
    }

    fn init_loop(hammer: &mut Hammer, mut line_number: (usize, usize), vec: &mut Vec::<String>)-> Result<(usize, usize), String>{
        line_number.1 += 1;
        loop{
            if line_number.1 == vec.len(){
                break;
            }
            let mut line = split(&vec[line_number.1], " ");
            line_number.0 += clean_line(&mut line);
            match line.len(){
                2 => {
                    hammer.define_new_var(line[1].clone(), line[0].clone())?;
                }
                1 =>{
                    if line[0] == "TEXT"{
                        break;
                    }else{
                        return Err(format!("Line {}: Syntax error.", line_number.0));
                    }
                }
                _ => {
                    if line.len() != 0  {
                        return Err(format!("Line {}: Syntax error.", line_number.0));
                    }
                }
            } 
            line_number.1 += 1;
        }
        Ok(line_number)
    }

    fn instruct_loop(hammer: &mut Hammer, mut line_number: (usize, usize), vec: &mut Vec::<String>) -> Result<(), String>{
        line_number.1 += 1;
        loop{
            if line_number.1 == vec.len(){
                break;
            }
            let mut line = split(&vec[line_number.1], " ");
            line_number.0 += clean_line(&mut line);
            println!("{}", hammer.memory.base_type.0.get_mut(&3).unwrap().value);
            if line.len() != 0{
                if !handle_affectation(hammer, &line, line_number.0)? {
                    // let func_name = line.remove(0);
                    // if hammer.func_exists(&func_name){
                    //     hammer.is_valid_parameter(&hammer.func_list[&func_name], line, line_number.0)?;
                    // }
                    // line_number.1 += 1;
                }
            } 
            println!("{}", hammer.memory.base_type.0.get_mut(&3).unwrap().value);
            line_number.1 += 1;
        }
        Ok(())        
    }

    fn handle_affectation(hammer: &mut Hammer, line: &Vec::<String>, line_number: usize) -> Result<bool, String> {
        let string_line = line.join(" ");
        if string_line.contains("="){
            let split = split(&string_line, "=");
            if split.len() != 2{
                return Err(format!("Line {}: Invalid syntax.", line_number));
            }
            let var1 = setup_var(split[0].clone());
            let mut var2 = setup_var(split[1].clone());
            if hammer.var_exists(&var1){
                let addr = hammer.defined_var_list[&var1].val();
                let var_def: VariableDefinition;
                if !is_value(&var2){
                    if hammer.var_exists(&var2){
                        let var_addr = hammer.defined_var_list[&var2].val();
                        var_def = hammer.addr_list[&var_addr].clone();
                        match var_def.type_var.id{
                            0 => {var2 = format!("{}", hammer.memory.base_type.0[&var_addr].value)}
                            1 => {var2 = format!("{}", hammer.memory.base_type.1[&var_addr].value)}
                            _ => panic!("Impossible type")
                        }
                    }else{
                        return Err(format!("Line {}: The variable {} doesn't exists.", line_number, var2));
                    }
                }else{
                    var_def = VariableDefinition{
                        name: String::from("tmp"), 
                        type_var: hammer.addr_list[&addr].type_var.clone()
                    };
                    
                }
                hammer.try_to_affect(*addr, var_def, var2, line_number)
            }else{
                return Err(format!("Line {}: The variable {} doesn't exists.", line_number, var1));
            }
            
        }else{
            Ok(false)
        }
    }

    fn setup_var(mut var: String) -> String{
        while var.len() != 0 && var.chars().nth(0).unwrap() == ' '{
            var.remove(0);
        }
        let mut s = var.len();
        while var.len() != 0 && var.chars().nth(s-1).unwrap() == ' '{
            var.remove(s-1);
            s -= 1;
        }
        return var;
    }

    fn clean_line(line: &mut Vec::<String>) -> usize{
        let mut res = 0;
        let mut i = 0;
        while i < line.len(){
            res += count_occur(&line[i], '\n') as usize;
            if line[i] == String::from("") {
                line.remove(i);
            }else{
                while line[i].contains("\n"){
                    line[i] = line[i].replace("\n", "");
                }
                i += 1;
            }
        }
        return res;
    }


    fn get_assembly_txt(hammer: Hammer) -> Result<String, String>{
        Ok(format!("{:?} {:?}", hammer.func_list.keys(), hammer.defined_var_list.keys()))
    }
}