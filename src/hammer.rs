#[allow(dead_code)]
pub mod hammer{
    use std::collections::HashMap;
    use crate::stack::Stack;
    use crate::tools::tools::{Tools, TextFile, split, count_occur, is_par};

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

    struct AsmType{
        long: &'static str,
        short: &'static str,
        registre: &'static str
    }

    struct VariableDefinition{
        name: String,
        decal: i32,
        type_var: Type,
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
    
    struct Affectation{
        addr: i32,
        exp: Vec::<(i32, u8)>,
    }

    struct MacroCall{
        macro_name: String,
        args: Vec::<(i32, i8)>
    }

    struct Instruction {
        macro_call: Option<MacroCall>,
        aff: Option<Affectation>,
    }
    


    struct Hammer{
        tools: Tools,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<i32>>,
        addr_list: HashMap::<i32, VariableDefinition>,
        func_list: HashMap::<String, Function>,
        macro_list: HashMap::<String, Macro>,
        inst_list: Vec::<Instruction>,
        jumps_stack: Stack<Vec::<i32>>,
        size: HashMap<i32, AsmType>
    }

    impl Hammer{
        pub fn new()->Hammer{
            let mut res = Hammer{
                tools: Tools::new(),
                type_list: HashMap::<String, Type>::new(),
                defined_var_list: HashMap::<String, Stack::<i32>>::new(),
                addr_list: HashMap::<i32, VariableDefinition>::new(),
                func_list: HashMap::<String, Function>::new(),
                macro_list: HashMap::<String, Macro>::new(),
                inst_list: Vec::new(),
                jumps_stack: Stack::new(),
                size: HashMap::<i32, AsmType>::new()
            };
            res.init_size();
            res.init_dispo_type();
            res.init_dispo_func();
            res.init_dispo_macro();
            return res
        }

        fn init_size(&mut self){
            self.size.insert(1, AsmType{
                short: "db", 
                long: "byte",
                registre: "al"
            });
            self.size.insert(2, AsmType{
                short: "dw", 
                long: "word",
                registre: "ax"
            });
            self.size.insert(4, AsmType{
                short: "dd", 
                long: "dword",
                registre: "eax"
            });
            self.size.insert(8, AsmType{
                short: "dq", 
                long: "qword",
                registre: "rax"
            });
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


        pub fn is_valid_type(&self, name: String) -> Result<(String, i32), String>{
            if name.len() > 3{
                let mut rev = name.chars().rev();
                let end = (rev.next().unwrap(), rev.next().unwrap(), rev.next().unwrap());
                if end.0 == ']' && end.2 == '[' {
                    match str::parse::<i32>(&String::from(end.1)){
                        Ok(number) => return self.type_exists(number, rev.rev().collect::<String>()),
                        _ => return Err(format!("Error: {} found when a number was await.", end.1))
                    }
                    
                }else if end.1 == '[' && end.0 == ']' {
                    return Err(String::from("You have to specify the size of your array."));
                }
                else{
                    self.type_exists(1, name)
                }
            }else{
                self.type_exists(1, name)
            }
            
        }

        fn type_exists(&self, size: i32, name: String) -> Result<(String, i32), String>{
            if self.type_list.contains_key(&name){
                Ok((name, size))
            }else{
                Err(format!("{} is not a valid type.", name))
            }
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

        pub fn get_addr(&self, name: &str) -> i32{
            *self.defined_var_list[name].val()
        }

        pub fn get_var_def_by_name(&self, name: &str) -> &VariableDefinition{
            return &self.addr_list[&self.get_addr(name)]
        }

        pub fn define_new_var(&mut self, name: String, mut type_var: String, data_file: Option<&mut TextFile>) -> Result<(), String>{
            let result_type_analyse = self.is_valid_type(type_var)?;
            type_var = result_type_analyse.0;
            let size_value = result_type_analyse.1;
            if !self.tools.is_valid_name(&name){
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
                match data_file{
                    Some(_file) => return Err(format!("There is already a gloable variable with the name {}", name)),
                    _ => self.defined_var_list.get_mut(&name).unwrap().push(addr)
                }
            }

            let var_def: VariableDefinition = VariableDefinition{
                name: name.clone(),
                decal: 0,
                type_var: self.type_list[&type_var].clone()
            };
            
            match data_file{
                Some(file) =>{
                    file.push(&format!("{}: {}", name, self.size[&var_def.type_var.size].short));
                    for _ in 0..size_value{
                        file.push(" 0,")
                    }
                }
                _ => {}
            }

            self.addr_list.insert(
                addr,
                var_def
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
                let var_addr = self.get_addr(&param[i]);
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
        
        pub fn get_size_def(&self, addr: i32) -> &AsmType{
            &self.size[&self.addr_list[&addr].type_var.size]
        }

        fn get_extract_string(&self, addr: i32) -> String{
            let size_def: &AsmType = self.get_size_def(addr);
            let var_def: &VariableDefinition = &self.addr_list[&addr];
            return String::from(format!("{}[{} + {}*{}]", size_def.long, var_def.name, var_def.decal, var_def.type_var.size))
        }

    }

    pub fn compile_txt(input: String) -> Result<(), String>{
        let mut hammer: Hammer = Hammer::new();
        reset_asm_file()?;
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
        let mut data_file = TextFile::new(String::from("asm/data.asm"))?;
        loop{
            if line_number.1 == vec.len(){
                break;
            }
            let mut line = split(&vec[line_number.1], " ");
            line_number.0 += clean_line(&mut line);
            match line.len(){
                2 => {
                    hammer.define_new_var(line[1].clone(), line[0].clone(), Some(&mut data_file))?;
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
                                        args.push((hammer.get_addr(arg), 1));
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
                        //name = setup_var(name);
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
        let string_line = line.join(" ");
        if string_line.contains("="){
            let split = split(&string_line, "=");
            if split.len() != 2{
                return Err(format!("Line {}: Invalid syntax.", line_number));
            }
            let var1 = setup_var(split[0].clone());
            let var2 = setup_var(split[1].clone());
            if hammer.var_exists(&var1){
                let addr = hammer.get_addr(&var1);
                let exp = build_aff_vec(hammer, var2, line_number)?;
                hammer.inst_list.push(
                    Instruction{
                        macro_call: None,
                        aff: Some(Affectation{
                            addr: addr,
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

    fn build_aff_vec(hammer: &Hammer, string_exp: String, line_number: usize) -> Result<Vec::<(i32, u8)>, String>{
        if string_exp == String::from(""){
            return Err(format!("Line {}: Syntax error.", line_number));
        }
        let mut exp = Vec::<String>::new();
        let mut current_element = String::new(); 
        for chara in string_exp.chars(){
            if chara != ' '{
                if hammer.tools.is_operator(&String::from(chara)) || is_par(chara){
                    if current_element != ""{
                        exp.push(current_element);
                    }
                    exp.push(String::from(chara));
                    current_element = String::new();
                }else{
                    current_element.push(chara);
                }
            }
        }
        exp.push(current_element);
        exp = hammer.tools.convert_in_postfix_exp(exp);
        let mut res = Vec::<(i32, u8)>::new();
        for elt in exp.iter(){
            add_element_in_aff_exp(hammer, &elt, &mut res, line_number)?;
        }
        Ok(res)
    }

    fn add_element_in_aff_exp(hammer: &Hammer, current_element: &str, exp: &mut Vec::<(i32, u8)>, line_number: usize) -> Result<(), String>{
        if current_element == String::from(""){
            return Err(format!("Line {}: Syntax error.", line_number));
        }
        match str::parse::<i32>(&current_element){
            Ok(number) => {
                exp.push((number as i32, 0));
            }
            _ => {
                if hammer.tools.is_valid_name(&current_element){
                    if hammer.var_exists(&current_element){
                        exp.push((hammer.get_addr(current_element), 1));
                    }else{  
                        return Err(format!("Line {}: Variable {} doesn't exists.", line_number, current_element));
                    }
                }else if hammer.tools.is_operator(&current_element){
                    exp.push((current_element.chars().next().unwrap() as i32, 2))
                }else{
                    return Err(format!("Line {}: Syntax error.", line_number));
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


    fn get_assembly_txt(mut hammer: Hammer) -> Result<(), String>{
        let mut _macro_file = TextFile::new(String::from("asm/macros.asm"))?;
        let mut script_file = TextFile::new(String::from("asm/script.asm"))?;
        script_file.push(&text_asm(&mut hammer)?);
        Ok(())
    }

    fn text_asm(hammer: &mut Hammer) -> Result<String, String>{
        let mut result = String::new();
        for inst in hammer.inst_list.iter(){
            match &inst.aff{
                Some(aff) => {
                    result = evaluate_exp(hammer, &aff.exp, result);
                    result.push_str(&format!("mov {}, {}\n", hammer.get_extract_string(aff.addr), hammer.get_size_def(aff.addr).registre));
                }
                _ =>{
                    let macro_call: &MacroCall = inst.macro_call.as_ref().unwrap();
                    let mut macro_call_s = String::from(format!("{} ", macro_call.macro_name));
                    for arg in &macro_call.args{
                        match arg.1{
                            0 => {
                                macro_call_s.push_str(&format!("{}", arg.0));
                            }
                            _ =>{
                                let var_def = &hammer.addr_list[&arg.0];
                                macro_call_s.push_str(&format!("[{}]", var_def.name));
                            }
                        }
                    }
                    result.push_str(&macro_call_s);
                    result.push_str("\n");
                }
            }
        }
        Ok(result)
    }
    
    

    fn evaluate_exp(hammer: &Hammer, exp: &Vec<(i32, u8)>, mut res: String) -> String{
        for elt in exp{
            if elt.1 == 2{
                res.push_str(&format!("pop r10\npop r11\nmov r12, {}\ncall _operation\npush rax\n", elt.0));
            }else{
                match elt.1{
                    1 => {
                        let size_def = hammer.get_size_def(elt.1 as i32);
                        res.push_str(&format!("xor rax, rax\nmov {}, {}\npush rax\n", size_def.registre, hammer.get_extract_string(elt.0 as i32)));
                    }
                    _ =>{
                        res.push_str(&format!("mov rax, {}\npush rax\n", elt.0));
                    }
                }       
            }
        }
        res.push_str("pop rax\n");
        res = res.replace("push rax\npop rax\n", "");
        res
    }


    fn reset_asm_file() -> Result<(), String>{
        let mut macro_file = TextFile::new(String::from("asm/macros.asm"))?;
        let mut data_file = TextFile::new(String::from("asm/data.asm"))?;
        let mut script_file = TextFile::new(String::from("asm/script.asm"))?;

        let mut base_macro_file: TextFile = TextFile::new(String::from("asm/base_files/base_macros.asm"))?;
        let mut base_data_file = TextFile::new(String::from("asm/base_files/base_data.asm"))?;
        let mut base_script_file = TextFile::new(String::from("asm/base_files/base_script.asm"))?;

        macro_file.reset(&base_macro_file.get_text());
        script_file.reset(&base_script_file.get_text());
        data_file.reset(&base_data_file.get_text());
        Ok(())
    }
}