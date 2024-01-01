#[allow(dead_code)]
pub mod hammer{
    use std::collections::HashMap;
    use crate::stack::Stack;
    use crate::tools::tools::{Tools, split, count_occur};

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

    struct Macro{
        name: String,
        nb_arg: usize,
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

    
    struct Affectation{
        addr: i32,
        exp: Vec::<(i32, i8)>,
    }

    struct MacroCall{
        macro_name: String,
        args: Vec::<(i32, i8)>
    }

    struct Instruction {
        macro_call: Option<MacroCall>,
        aff: Option<Affectation>,
    }
    


    struct Hammer<'a>{
        tools: Tools,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<i32>>,
        addr_list: HashMap::<i32, VariableDefinition>,
        func_list: HashMap::<String, Function>,
        macro_list: HashMap::<String, Macro>,
        inst_list: Vec::<Instruction>,
        jumps_stack: Stack<Vec::<i32>>,
        authorized_char_for_variable: &'a str,
        operators: &'a str 
    }

    impl<'a> Hammer<'a>{
        pub fn new()->Hammer<'a>{
            let mut res = Hammer{
                tools: Tools::new(),
                type_list: HashMap::<String, Type>::new(),
                defined_var_list: HashMap::<String, Stack::<i32>>::new(),
                addr_list: HashMap::<i32, VariableDefinition>::new(),
                func_list: HashMap::<String, Function>::new(),
                macro_list: HashMap::<String, Macro>::new(),
                inst_list: Vec::new(),
                jumps_stack: Stack::new(),
                authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
                operators: "+-*"
            };
            res.init_dispo_type();
            res.init_dispo_func();
            res.init_dispo_macro();
            return res
        }

        pub fn jump_out(&mut self){
            let variable_to_destroy: Vec::<i32> = self.jumps_stack.pop();
            for addr in variable_to_destroy{
                let def = self.addr_list.remove(&addr).unwrap();
                self.defined_var_list.get_mut(&def.name).unwrap().pop();
            }
        }

        fn init_dispo_func(&mut self){
        }

        fn init_dispo_macro(&mut self){
            self.macro_list.insert(
                String::from("display_number"), 
                Macro{
                    name: String::from("display_number"), 
                    nb_arg: 1, 
                }
            );
            self.macro_list.insert(
                String::from("exit"), 
                Macro{
                    name: String::from("exit"), 
                    nb_arg: 1
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

        pub fn is_operator(&self, x: String) -> bool{
            self.operators.contains(&x)
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

        pub fn macro_exists(&self, name: &str) -> bool{
            self.macro_list.contains_key(name)
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
            if line_number.1 == vec.len() -1{
                break;
            }
            let mut line_split = split(&vec[line_number.1], " ");
            line_number.0 += clean_line(&mut line_split);
            if line_split.len() != 0{
                if !handle_affectation(hammer, &line_split, line_number.0)? {
                    let mut line = line_split.join(" ");
                    if line.pop() != Some(')'){
                        return Err(format!("Line {}: Syntax error.", line_number.0));   
                    }
                    let mut split_par = line.split("(");
                    let mut name = String::from(split_par.next().unwrap());
                    
                    if name.chars().next().unwrap() == '!'{
                        name.remove(0);
                        name = setup_var(name);
                        if !hammer.macro_exists(&name){
                            return Err(format!("Line {}: The macro {} doesn't exists.", line_number.0, name));
                        }
                        line = match split_par.next() {
                            Some(rest_of_line)=> String::from(rest_of_line),
                            _ => return Err(format!("Line {}: Parenthesis missing.", line_number.0))
                        };
                        let mut args = Vec::<(i32, i8)>::new();
                        line = line.replace(" ", "");
                        for arg in line.split(","){
                            if arg == ""{
                                break;
                            }
                            match str::parse::<i32>(&arg){
                                Ok(number) =>{args.push((number, 0))}
                                Err(_e) => {
                                    if hammer.var_exists(arg){
                                        args.push((*hammer.defined_var_list[arg].val(), 1));
                                    }else{
                                        return Err(format!("Line {}: bad argument found, {}", line_number.0, arg));
                                    }
                                }
                            }
                        }
                        let nb_arg_expected = hammer.macro_list[&name].nb_arg;
                        if args.len() != nb_arg_expected{
                            return Err(format!("Line {}: Found {} arguments when {} was expected", line_number.0, args.len(), nb_arg_expected));
                        }
                        hammer.inst_list.push({
                            Instruction{
                                macro_call: Some(MacroCall{
                                    macro_name: name,
                                    args: args 
                                }),
                                aff: None
                            }
                        });
                        
                    }else{
                        name = setup_var(name);
                        println!("Its a function: {name}",)
                    }
                    let func_name = line_split.remove(0);
                    if hammer.func_exists(&func_name){
                        hammer.is_valid_parameter(&hammer.func_list[&func_name], line_split, line_number.0)?;
                    }
                }
            } 
            line_number.1 += 1;
        }
        Ok(())        
    }


    fn handle_affectation(hammer: &mut Hammer, line: &Vec::<String>, line_number: usize) -> Result<bool, String> {
        println!("{}: {:?}", line_number, line);
        let string_line = line.join(" ");
        if string_line.contains("="){
            let split = split(&string_line, "=");
            if split.len() != 2{
                return Err(format!("Line {}: Invalid syntax.", line_number));
            }
            let var1 = setup_var(split[0].clone());
            let var2 = setup_var(split[1].clone());
            if hammer.var_exists(&var1){
                let addr = hammer.defined_var_list[&var1].val();
                let exp = build_aff_vec(hammer, var2, line_number)?;
                hammer.inst_list.push(
                    Instruction{
                        macro_call: None,
                        aff: Some(Affectation{
                            addr: *addr,
                            exp: exp      
                    })
                    }
                );
                Ok(true)
            }else{
                return Err(format!("Line {}: The variable {} doesn't exists.", line_number, var1));
            }
            
        }else{
            Ok(false)
        }
    }

    fn build_aff_vec(hammer: &Hammer, string_exp: String, line_number: usize) -> Result<Vec::<(i32, i8)>, String>{
        if string_exp == String::from(""){
            return Err(format!("Line {}: Syntax error.", line_number));
        }
        let mut exp = Vec::<(i32, i8)>::new();
        let mut current_element = String::new(); 
        for chara in string_exp.chars(){
            if chara != ' '{
                if hammer.is_operator(String::from(chara)){
                    add_element_in_aff_exp(hammer, &current_element, &mut exp, line_number)?;
                    exp.push((chara as i32, 2));
                    current_element = String::new();
                }else{
                    current_element.push(chara);
                }
            }
        }
        add_element_in_aff_exp(hammer, &current_element, &mut exp, line_number)?;
        Ok(exp)
    }

    fn add_element_in_aff_exp(hammer: &Hammer, current_element: &str, exp: &mut Vec::<(i32, i8)>, line_number: usize) -> Result<(), String>{
        if current_element == String::from(""){
            return Err(format!("Line {}: Syntax error.", line_number));
        }
        match str::parse::<i32>(&current_element){
            Ok(number) => {
                exp.push((number as i32, 0));
            }
            _ => {
                if hammer.is_valid_name(&current_element){
                    if hammer.var_exists(&current_element){
                        exp.push((*hammer.defined_var_list[current_element].val(), 1));
                    }else{  
                        return Err(format!("Line {}: Variable {} doesn't exists.", line_number, current_element));
                    }
                }else{
                    return Err(format!("Line {}: Syntax erro.", line_number));
                }
                
            }
        }
        Ok(())
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
        Ok(format!("{:?} {:?}", hammer.inst_list[1].aff.as_ref().unwrap().exp, hammer.defined_var_list.keys()))
    }
}