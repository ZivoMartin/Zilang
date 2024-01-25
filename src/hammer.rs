#[allow(dead_code)]
pub mod hammer{
    use std::collections::HashMap;
    use crate::stack::Stack;
    use crate::tools::tools::*;
    use std::str;

    static POINTER: &str = "$";
    static POINTER_SIZE: u32 = 4;
    static MAX_STARS: u32 = 10000; 
    static mut LINE_NUMBER: (usize, usize) = (1, 0);

    fn get_ln() -> usize {
        unsafe{
            return LINE_NUMBER.0
        }
    }

    fn get_in() ->usize {
        unsafe {
            return LINE_NUMBER.1
        }
    } 

    fn inc_in(x: usize) {
        unsafe{
            LINE_NUMBER.1 += x;
        }
    }

    fn inc_ln(x: usize) {
        unsafe{
            LINE_NUMBER.0 += x;
        }
    }

    

    #[derive(Debug)]
    struct Type {
        name: String,
        id: i32,
        size: i32,
        stars: u32
    }

    impl Type{
        pub fn clone(&self) -> Type{
            Type{
                name: self.name.clone(),
                id: self.id,
                size: self.size,
                stars: self.stars
            }
        } 
    }

    struct Function {
        args: Vec::<VariableDefinition>,
        return_type: Type
    }

    impl Function {

        pub fn clone(&self) -> Function {
            let mut args_cloned = Vec::<VariableDefinition>::new();
            for elt in self.args.iter() {
                args_cloned.push(elt.clone());
            }
            Function {
                args: args_cloned,
                return_type: self.return_type.clone()
            }
        }

    }
    
    struct AsmType{
        long: &'static str,
        short: &'static str,
        register: &'static str,
        mov: &'static str
    }
    #[derive(Debug)]
    struct VariableDefinition{
        name: String,
        type_var: Type,
    }
    
    impl VariableDefinition {
        pub fn clone(&self) -> VariableDefinition {
            VariableDefinition {
                name: self.name.clone(),
                type_var: self.type_var.clone()
            }
        }
    }

    struct MacroCall{
        macro_name: String,
        args: Vec::<Vec::<Token>>
    }

    struct Jump{
        vars: Vec::<i32>, 
        stack_index: u32,
        action: (fn(&mut Hammer, String), String),
        bloc_index: u32
    }

    impl Jump {

        fn new(stack_index: u32, action: (fn(&mut Hammer, String), String), bloc_index: u32) -> Self {
            Jump{
                vars: Vec::new(),
                stack_index: stack_index,
                action: action,
                bloc_index: bloc_index
            }
        }

    }

    enum Interp {
        Function,
        Variable,
        Value,
        Operator
    }
    struct Token {
        val: i32,
        nb_stars: i32,
        squares: Option<Vec::<Vec<Token>>>,
        func_dec: Option<String>,
        interp: Interp
    }
    
    impl Token {

        pub fn new_val(val: i32) -> Token{
            Token{val: val, squares: None, func_dec: None, nb_stars: 0, interp: Interp::Value}
        }
        pub fn new_op(op: i32) -> Token {
            Token{val: op, squares: None, func_dec: None, nb_stars: 0, interp: Interp::Operator}
        }
        pub fn new_func(func: String) -> Token {
            Token{val: 0, squares: None, func_dec: Some(func), nb_stars: 0, interp: Interp::Function}
        }
    }

    struct Hammer{
        tools: Tools,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<i32>>,
        addr_list: HashMap::<i32, VariableDefinition>,
        macro_list: HashMap::<String, u32>,
        keyword_list: HashMap::<String, fn(&mut Hammer, &String) -> Result<(), String>>,
        func_list: HashMap::<String, Function>,
        jumps_stack: Stack<Jump>,
        size: HashMap<u32, AsmType>,
        stack_index: u32,
        txt_stack: Stack<String>,
        blocs_index: u32,
        loop_index_stack: Stack<u32>,
        cond_index_stack: Stack<u32>
    }


    impl Hammer{
        pub fn new()->Hammer{
            let mut res = Hammer{
                tools: Tools::new(),
                type_list: HashMap::<String, Type>::new(),
                defined_var_list: HashMap::<String, Stack::<i32>>::new(),
                addr_list: HashMap::<i32, VariableDefinition>::new(),
                macro_list: HashMap::<String, u32>::new(),
                keyword_list: HashMap::<String, fn(&mut Hammer, &String) -> Result<(), String>>::new(),
                func_list: HashMap::<String, Function>::new(),
                jumps_stack: Stack::new(),
                size: HashMap::<u32, AsmType>::new(),
                stack_index : 0,
                txt_stack: Stack::new(),
                blocs_index: 1,
                loop_index_stack: Stack::new(),
                cond_index_stack: Stack::new()
            };
            res.init_size();
            res.init_dispo_type();
            res.init_dispo_macro();
            res.init_dispo_keyword();
            return res
        }

        fn init_size(&mut self){
            self.size.insert(1, AsmType{
                short: "db", 
                long: "byte",
                register: "al",
                mov: "movzx"
            });
            self.size.insert(2, AsmType{
                short: "dw", 
                long: "word",
                register: "ax",
                mov: "movsx"
            });
            self.size.insert(4, AsmType{
                short: "dd", 
                long: "dword",
                register: "eax",
                mov: "movsx"
            });
            self.size.insert(8, AsmType{
                short: "dq", 
                long: "qword",
                register: "rax",
                mov: "mov"
            });
        }

        pub fn init_dispo_keyword(&mut self) {
            self.keyword_list.insert(String::from("break"), break_keyword);
            self.keyword_list.insert(String::from("continue"), continue_keyword);
            self.keyword_list.insert(String::from("if"), if_keyword);
            self.keyword_list.insert(String::from("else"), else_keyword);
            self.keyword_list.insert(String::from("loop"), loop_keyword);
            self.keyword_list.insert(String::from("while"), while_keyword);
            self.keyword_list.insert(String::from("while"), while_keyword);
            self.keyword_list.insert(String::from("func"), func_keyword);
            self.keyword_list.insert(String::from("return"), return_keyword);
        }

        pub fn jump_out(&mut self){
            let top_stack = self.jumps_stack.val();
            if !self.loop_index_stack.is_empty() && *self.loop_index_stack.val() == top_stack.bloc_index as u32{
                self.loop_index_stack.pop();
            }
            self.stack_index = top_stack.stack_index as u32;
            let variable_to_destroy: &Vec::<i32> = &top_stack.vars; 
            for addr in variable_to_destroy{
                let def = self.addr_list.remove(&addr).unwrap();
                let var_stack: &mut Stack<_> = self.defined_var_list.get_mut(&def.name).unwrap(); 
                var_stack.pop();
                if var_stack.is_empty() {
                    self.defined_var_list.remove(&def.name);
                }
            }
            top_stack.action.0(self, top_stack.action.1.clone());
            self.jumps_stack.pop();
        }

        fn push_txt(&mut self, txt: &str) {   
            self.txt_stack.val_mut().push_str(txt);
        }

        fn replace_txt(&mut self, txt1: &str, txt2: &str) {
            *self.txt_stack.val_mut() = self.txt_stack.val_mut().replace(txt1, txt2); 
        }

        fn init_dispo_macro(&mut self){
            self.macro_list.insert(String::from("dn"), 1);
            self.macro_list.insert(String::from("exit"), 1);
        }

        fn init_dispo_type(&mut self){
            self.new_type(String::from("int"), 4);
            self.new_type(String::from("char"), 1);
            self.new_type(String::from("void"), 0);
        } 

        fn new_type(&mut self, name: String, size: i32){
            self.type_list.insert( name.clone(), Type{name: name, id: self.type_list.len() as i32, size: size, stars: 0});   
        }

        fn define_new_function(&mut self, name: String, args: Vec::<VariableDefinition>, return_type: Type) -> Result<(), String> {
            if self.func_exists(&name) {
                return Err(format!("Line {}: The function {} already exists", get_ln(), name))
            }else{
                self.func_list.insert(name, Function{args: args, return_type: return_type});
            }
            Ok(())
        }

        fn func_exists(&self, name: &str) -> bool {
            self.func_list.contains_key(name)
        }       
        
        fn type_exists(&self, mut name: &mut String) -> Result<Type, String>{
            let nb_stars = extract_end_char(&mut name, '*');
            if self.type_list.contains_key(name){
                let mut result = self.type_list[name].clone();
                result.stars = nb_stars;
                return Ok(result)
            }else{
                Err(format!("{} is not a valid type.", name))
            }
        }

        pub fn var_exists(&self, name: &str) -> bool{
            self.defined_var_list.contains_key(name)
        }

        pub fn macro_exists(&self, name: &str) -> bool{
            self.macro_list.contains_key(name)
        }


        pub fn keyword_exists(&self, name: &str) -> bool {
            self.keyword_list.contains_key(name)
        }

        pub fn call_keyword(&mut self, name: &str, rest_of_line: &String) -> Result<(), String> {
            self.keyword_list[name](self, rest_of_line)
        }

        pub fn get_addr(&self, name: &str) -> i32{
            *self.defined_var_list[name].val()
        }

        pub fn get_var_def_by_name(&self, name: &str) -> &VariableDefinition{
            return &self.addr_list[&self.get_addr(name)]
        }

        pub fn define_new_var(&mut self, name: String, type_var: Type){
            let addr = self.insert_var_in_memory(&name, type_var);
            self.jumps_stack.val_mut().vars.push(addr);
            if !self.var_exists(&name){
                let mut new_stack = Stack::<i32>::new();
                new_stack.push(addr);
                self.defined_var_list.insert(
                    name,
                    new_stack
                );    
            }else{
                self.defined_var_list.get_mut(&name).unwrap().push(addr)
            }
        }

        fn insert_var_in_memory(&mut self, name: &str, type_var: Type) -> i32 {
            let addr = self.stack_index as i32;
            self.stack_index += type_var.size as u32;

            let var_def: VariableDefinition = VariableDefinition{
                name: String::from(name),
                type_var: type_var
            };
            
            self.addr_list.insert(
                addr,
                var_def
            );
            addr
        }
        
        pub fn get_size_def(&self, addr: i32) -> &AsmType{
            &self.size[&(self.addr_list[&addr].type_var.size as u32)]
        }

        pub fn is_valid_name(&self, name: &str) -> Result<(), String>{
            if name != "" && name.chars().nth(0).unwrap() == '_'{
                return Err(String::from("A name can't start with '_' it's a vulcain private use."));
            }else if !self.tools.is_valid_name(name){
                return Err(format!("{} is not a valid name for a variable.", name))
            }
            Ok(())
        }


    }

    pub fn compile_txt(input: String) -> Result<(), String>{
        let mut hammer: Hammer = Hammer::new();
        reset_asm_file()?;
        let mut vec = split(&input, ";");
        begin_loop(&mut vec)?;
        if get_in() == vec.len() {
            return Err(format!("You can't compile without text"));
        } 
        instruct_loop(&mut hammer, &mut vec)?;
        let mut script_file = TextFile::new(String::from("asm/script.asm"))?;
        script_file.push(&hammer.txt_stack.val());
        Ok(())
    }

    fn begin_loop(vec: &mut Vec::<String>) -> Result<(), String>{
        loop{
            if get_in() == vec.len(){
                return Ok(())
            }
            let mut line = split(&vec[get_in()], " ");
            inc_ln(clean_line(&mut line));
            match line.len(){
                1 => {
                    if line[0] == "INIT" || line[0] == "TEXT"{
                        return Ok(())
                    }else if line[0] != "MAIN"{
                        return Err(format!("Line {}: Syntax error.", get_ln()));
                    }
                }
                _ => {
                    if line.len() != 0  {
                        return Err(format!("Line {}: syntax error.", get_ln()));
                    }
                }
            }
            inc_in(1);
        }
    }

    fn end_prog(_hammer: &mut Hammer, _s: String) {}
    
   
    fn setup_inst(inst: &mut String){
        let mut i: usize = 0;
        for chara in inst.clone().chars() {
            if chara == '=' {
                inst.insert(i+1, ' ');
                inst.insert(i, ' ');
                break;
            }
            i += 1;
        }
        *inst = inst.trim().to_string();
    }

    fn annalyse_inst_behind_bracket(hammer: &mut Hammer, inst: String) -> Result<(), String> {
        let first_word = inst.split(" ").next().unwrap();
        if hammer.keyword_exists(first_word) {
            return hammer.keyword_list[first_word](hammer, &String::from(&inst[first_word.len()..inst.len()]).trim().to_string())
        }else if inst != "" {
            return Err(format!("Line {}: Syntax error.", get_ln()));
        }else{
            end_of_inst(hammer, String::from(""));
        }
        Ok(())
    }

    fn handle_tab_type_dec(hammer: &mut Hammer, var: &mut Vec<&str>, type_var: &mut Type) -> Result<(), String> {
        let tab_addr = hammer.stack_index;
        let mut previous_data: (u32, u32) = (1, hammer.stack_index);
        for i in 1..var.len() {
            if var[i] == "" || var[i].chars().last().unwrap() != ']'{
                return Err(format!("Line {}: You forgot to close with '['", get_ln()))
            }
            let stack_index = hammer.stack_index;
            match str::parse::<u32>(&var[i][0..var[i].len()-1]) {
                Ok(tab_size) => {
                    type_var.stars += 1;
                    let size: u32;
                    if i != var[i].len()-1 {
                        size = POINTER_SIZE;
                    }else{
                        size = type_var.size as u32;
                    }
                    for j in 0..previous_data.0{
                        hammer.push_txt(&format!("mov {}[_stack + r15 + {} + {}*{}], {}\n", hammer.size[&size].long, previous_data.1, size, j, hammer.stack_index));
                        hammer.stack_index += size*tab_size;
                    }
                    previous_data.0 *= tab_size;
                },
                Err(_) => return Err(format!("Line {}: You didn't write a correct number between the brackets", get_ln()))
            }
            previous_data.1 = stack_index;
        }
        if var.len() != 1 {
            hammer.push_txt(&format!("mov eax, {}\nmov rcx, r15\nmov dword[_stack + ecx + eax], {}\n", hammer.stack_index, tab_addr));
        }
        Ok(())
    }


    fn tab_analyse(hammer: &Hammer, var_s: &mut String) -> Result<Vec::<Vec<Token>>, String>{
        let var: Vec::<&str> = var_s.split("[").collect();
        let mut res = Vec::<Vec<Token>>::new();
        for i in 1..var.len() {
            if var[i] == "" || var[i].chars().last().unwrap() != ']'{
                return Err(format!("Line {}: You forgot to close with '['", get_ln()))
            }
            res.push(build_aff_vec(hammer, String::from(&var[i][0..var[i].len()-1]), 0)?);
        }
        *var_s = String::from(var[0]);
        Ok(res)
    }

    fn instruct_loop(hammer: &mut Hammer, vec: &mut Vec::<String>) -> Result<(), String>{
        inc_in(1);
        hammer.jumps_stack.push(Jump::new(0, (end_prog, String::new()), 0));
        hammer.txt_stack.push(String::from("xor r15, r15\n"));
        loop{
            if get_in() == vec.len() -1{
                break;
            }

            let mut inst = vec[get_in()].clone();
            let mut line_split = split(&inst, " ");
            inc_ln(clean_line(&mut line_split));
            inst = line_split.join(" ");
            inst = inst.trim().to_string();
            if inst.starts_with("}") {
                inst.remove(0);
                vec[get_in()] = inst;
                hammer.jump_out();
                continue;
            } else if inst.contains("{") {
                let mut split_inst: Vec::<&str> = inst.split("{").collect();
                annalyse_inst_behind_bracket(hammer, split_inst[0].trim().to_string())?;
                split_inst.remove(0);
                vec[get_in()] = split_inst.join("{");
                continue;
            }else{
                setup_inst(&mut inst);
                line_split = split(&inst, " ");
                handle_instruction(hammer, inst, line_split)?;
                inc_in(1);
            }
            
        }
        Ok(())
    }

    fn handle_instruction(hammer: &mut Hammer, mut inst: String, mut line_split: Vec::<String>) -> Result<(), String> {
        if line_split.len() != 0{
            match hammer.type_exists(&mut line_split[0]) {
                Ok(mut type_var) => {
                    if line_split.len() < 2{
                        return Err(format!("Line {}: You should initialise a variable.", get_ln()))
                    }
                    let mut split_var: Vec::<&str> = line_split[1].split("[").collect();
                    hammer.is_valid_name(split_var[0])?;
                    handle_tab_type_dec(hammer, &mut split_var, &mut type_var)?;
                    hammer.define_new_var(String::from(split_var[0]), type_var);
                    if inst.contains("="){
                        line_split.remove(0);
                        handle_affectation(hammer, line_split.join(" "))?;
                    }
                }   

                _ => {

                    if inst.contains("="){
                        handle_affectation(hammer, inst)?;
                    }else if inst.starts_with("!"){
                        inst.remove(0);
                        handle_macros_call(hammer, inst)?; 
                    }else if hammer.keyword_exists(&line_split[0]){
                        hammer.call_keyword(&line_split.remove(0), &line_split.join(" "))?;
                    }else if hammer.func_exists(line_split[0].split("(").next().unwrap()) {
                        handle_func_call(hammer, inst)?;
                    }else{
                        return Err(format!("Line {}: Not implemented yet.", get_ln()));
                    }
                }
            }
        } 
        Ok(())
    }


    fn handle_func_call(hammer: &mut Hammer, mut call: String) -> Result<(), String> {
        
        let mut split_par: Vec::<String> = call.split("(").map(String::from).collect();
        if split_par.len() == 1 {
            return Err(format!("Line {}: You have to specifie args between parenthesis.", get_ln()))
        }
        let func_name = split_par.remove(0);
        let func = &hammer.func_list[&func_name].clone();
        call = split_par.join("(");
        if call.chars().rev().nth(0).unwrap() != ')' {
            return Err(format!("Line {}: Parenthesis never closes.", get_ln()));
        }
        call = String::from(&call[0..call.len()-1]);
        let mut split_virg: Vec::<String> = call.split(",").map(String::from).collect();
        if split_virg.len() == 1 && split_virg[0].trim().to_string() == "" {
            split_virg.remove(0);
        }
        if split_virg.len() != func.args.len() {
            return Err(format!("Line {}: We found {} elements in the call of the function {} but {} takes {} arguments.", get_ln(), split_virg.len(), func_name, func_name, func.args.len()));
        }
        let mut decal = 0;
        for (i, exp) in split_virg.iter().enumerate() {
            evaluate_exp(hammer, &build_aff_vec(hammer, exp.to_string(), func.args[i].type_var.stars)?)?;
            let size_def = &hammer.size[&(func.args[i].type_var.size as u32)];
            hammer.push_txt(&format!("mov {}[_stack + r15 + {} + {}], {}\n", size_def.long, hammer.stack_index, decal, size_def.register));
            decal += func.args[i].type_var.size;
        }
        hammer.push_txt(&format!("push r15\nmov r15, {}\ncall {}\npop r15\n", hammer.stack_index, func_name));
        Ok(())
    }

    fn handle_macros_call(hammer: &mut Hammer, mut line: String) -> Result<(), String>{
        if line.pop() != Some(')'){
            return Err(format!("Line {}: Syntax error.", get_ln()));   
        }
        let mut split_par = line.split("(");
        let mut name = String::from(split_par.next().unwrap());
        name = name.trim().to_string();
        if !hammer.macro_exists(&name){
            return Err(format!("Line {}: The macro {} doesn't exists.", get_ln(), name));
        }
        line = match split_par.next() {
            Some(rest_of_line)=> String::from(rest_of_line),
            _ => return Err(format!("Line {}: Parenthesis missing.", get_ln()))
        };
        let mut args = Vec::<Vec::<Token>>::new();
        line = line.replace(" ", "");
        for arg in line.split(","){
            if arg == ""{
                break;
            }
            args.push(build_aff_vec(hammer, String::from(arg), 0)?);
        }
        let nb_arg_expected = hammer.macro_list[&name];
        if args.len() != nb_arg_expected as usize{
            return Err(format!("Line {}: Found {} arguments when {} was expected", get_ln(), args.len(), nb_arg_expected));
        }
        insert_macro_call_in_txt(hammer, MacroCall{
            macro_name: name,
            args: args 
        })?;
        Ok(())
    }



    fn handle_affectation(hammer: &mut Hammer, line: String) -> Result<(), String> {

        let mut split = split(&line, "=");
        if split.len() < 2{
            return Err(format!("Line {}: Invalid syntax.", get_ln()));
        }
        let mut var1 = split[0].trim().to_string();
        split.remove(0);
        let nb_stars = get_prof_pointer(&mut var1, false)?;
        let tab_vec = tab_analyse(hammer, &mut var1)?;
        if hammer.var_exists(&var1){
            let right_exp = split.join("=").replace(" = ", "=");
            let addr = hammer.get_addr(&var1);
            let struct_addr = Token{val: addr, func_dec: None, squares: Some(tab_vec), nb_stars: nb_stars, interp: Interp::Variable};
            let stars_in_left_var = handle_variable_dereference(hammer, &struct_addr)?;
            hammer.push_txt("push rax\n");
            evaluate_exp(hammer, &build_aff_vec(hammer, right_exp, stars_in_left_var as u32)?)?;

            if stars_in_left_var != 0 {
                hammer.push_txt("pop rbx\nmov dword[_stack + rbx], eax\n");
            }else{
                let size_def = hammer.get_size_def(addr);
                hammer.push_txt(&format!("pop rbx\nmov {}[_stack + rbx], {}\n", size_def.long, size_def.register));
            }
            
            Ok(())
        }else {
            return Err(format!("Line {}: The variable {} doesn't exists.", get_ln(), var1));
        }
       
    }

    

    fn get_prof_pointer(var: &mut String, can_be_ref: bool) -> Result<i32, String>{
        let mut count: i32 = 0;
        while var.len() != 0 && (var.starts_with(POINTER) || var.starts_with("&")){
            if var.starts_with(POINTER) {
                count += 1;
            }else{
                if count == -1{
                    return Err(format!("Line {}: You tried to get the adress of an invalid thing.", get_ln()));
                }
                count -= 1;
            }
            var.remove(0);
        }
        if count == -1 && !can_be_ref {
            return Err(format!("Line {}: You can't put a reference here.", get_ln()));
        }
        Ok(count)
    }

    fn new_operator_separator(current_element: &mut String, exp: &mut Vec<String>, last: String, neg_count: &mut u32){
        if current_element != ""{
            if *neg_count % 2 == 0 {
                exp.push(current_element.to_string());
            }else{
                exp.push(String::from("-") + &current_element.to_string());
            }
            
        }
        exp.push(last);
        *current_element = String::new();
        *neg_count = 0;
    }

    fn format_string_exp(hammer: &Hammer, mut string_exp: String) -> String {
        for op in hammer.tools.get_op_iter(){
            string_exp = string_exp.replace(op, &format!("_{}_", op));
        }
        string_exp = string_exp.replace("(", &format!("_(_"));
        string_exp = string_exp.replace(")", &format!("_)_"));
        while string_exp.contains("__") {
            string_exp = string_exp.replace("__", "_");
        }
        string_exp = string_exp.replace("<_=", "<=_");
        string_exp = string_exp.replace(">_=", ">=_");
        string_exp
    }

    fn build_aff_vec(hammer: &Hammer, mut string_exp: String, nb_stars_await: u32) -> Result<Vec::<Token>, String>{
        string_exp = string_exp.trim().to_string();
        if string_exp == String::from("") || string_exp.contains("_") && is_in_a_string(&string_exp, "_".to_string()){
            return Err(format!("Line {}: Syntax error.", get_ln()));
        }
        string_exp = format_string_exp(hammer, string_exp);
        let exp: Vec::<String>  = string_exp.split("_").map(String::from).collect();
        let mut exp_res = tokenise_expression(hammer, exp)?;
        println!("{exp_res:?}");
        exp_res = hammer.tools.convert_in_postfix_exp(exp_res);
        let mut res = Vec::<Token>::new();
        for elt in exp_res.iter(){
            add_element_in_aff_exp(hammer, elt.to_string(), &mut res, nb_stars_await)?;
        }
        Ok(res)
    }

    fn tokenise_expression(hammer: &Hammer, exp: Vec::<String>) -> Result<Vec<String>, String> {
        let mut exp_res = Vec::<String>::new();
        let mut current_element = String::new(); 
        let mut func_dec = String::new();
        let mut cant_be_op = true;
        let mut neg_count: u32 = 0;
        let mut looking_for_function = -1;
        for i in 0..exp.len(){
            let word = exp[i].trim().to_string();
            if word != "" {
                if looking_for_function == -1 {
                    let is_op = hammer.tools.is_operator(&word); 
                    if is_op || hammer.tools.is_separator(&word){
                        if is_op {
                            if cant_be_op{
                                match &word as &str{
                                    "-" => {
                                        neg_count += 1;
                                    }
                                    _ => return Err(format!("Line {}: There is a bad operator in your expression.", get_ln()))
                                }   
                            }else{
                                new_operator_separator(&mut current_element, &mut exp_res, word, &mut neg_count);    
                                cant_be_op = true;
                            }
                        }else {
                            new_operator_separator(&mut current_element, &mut exp_res, word, &mut neg_count);
                            cant_be_op = false;
                        }
                    }else{
                        if hammer.func_exists(&word) {
                            func_dec = word;
                            looking_for_function = 0;
                        }else{
                            current_element.push_str(&word);
                        }
                        cant_be_op = false;
                    }
                }else{
                    func_dec.push_str(&word);
                    if word == ")" {
                        looking_for_function -= 1;
                        if looking_for_function == 0 {
                            exp_res.push(func_dec.clone());
                            func_dec = String::new();
                            looking_for_function = -1;
                        }
                    }else if word == "(" {
                        looking_for_function += 1;
                    }
                }
            }
        }
        if current_element != ""{
            if neg_count % 2 == 0 {
                exp_res.push(current_element);
            }else{
                exp_res.push(String::from("-") + &current_element);
            }
        }
        Ok(exp_res)
    }

    fn is_in_a_string(_exp: &String, _part: String) -> bool {
        false
    }

    fn add_element_in_aff_exp(hammer: &Hammer, mut current_element: String, exp: &mut Vec::<Token>, nb_stars_await: u32) -> Result<(), String>{
        if current_element == ""{
            return Err(format!("Line {}: Syntax error.", get_ln()));
        }
        if hammer.tools.is_operator(&current_element){ 
            exp.push(Token::new_op(hammer.tools.ascii_val(&current_element)))
        }else{
            match current_element.parse::<i32>(){
                Ok(number) => exp.push(Token::new_val(number)),
                Err(_e) => {
                    let mut nb_stars = get_prof_pointer(&mut current_element, true)?;
                    let var_name = current_element.split("[").next().unwrap();
                    if hammer.var_exists(&var_name){
                        let var_def = hammer.get_var_def_by_name(&var_name);
                        if (var_def.type_var.stars as i32) < nb_stars {
                            return Err(format!("Line {}: You tried to dereference the variable {} {} times but you only have the right to dereference it {} times", get_ln(), var_def.name, nb_stars, var_def.type_var.stars));
                        }
                        let tab_vec = tab_analyse(hammer, &mut current_element)?;
                        nb_stars += tab_vec.len() as i32;
                        if nb_stars_await != MAX_STARS+1 && var_def.type_var.stars as i32 - nb_stars != nb_stars_await as i32{
                            return Err(format!("Line {}: The two types are incompatibles.", get_ln()));
                        }
                        nb_stars -= tab_vec.len() as i32;
                        exp.push(Token{val: hammer.get_addr(&current_element), squares: Some(tab_vec), func_dec: None, nb_stars: nb_stars, interp: Interp::Variable});
                    }else if hammer.func_exists(current_element.split("(").next().unwrap()){
                        exp.push(Token::new_func(current_element));
                    }else{
                        match from_char_to_number(&current_element){
                            Some(val) => exp.push(Token::new_val(val as i32)),
                            _ => return Err(format!("Line {}: we found an incorrect value: {}", get_ln(), current_element))
                        }
                    }
                }
            }
        }
        Ok(())
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

    
    fn insert_macro_call_in_txt(hammer: &mut Hammer, macro_call: MacroCall) -> Result<(), String> {
        let mut i = 0;
        for arg in &macro_call.args{
            evaluate_exp(hammer, arg)?;
            hammer.push_txt(&format!("mov [_stack + r15 + {} + {}], rax\n", hammer.stack_index, i*8));
            i += 1;
        }
        hammer.push_txt(&format!("{} ", macro_call.macro_name));
        for j in 0..i {
            hammer.push_txt(&format!("[_stack + r15 + {} + {}] ", hammer.stack_index, j*8));
        }
        hammer.push_txt("\nxor rax, rax\n");
        for j in 0..i {
            hammer.push_txt(&format!("mov [_stack + r15 + {} + {}], rax\n", hammer.stack_index, j*8));
        }
        Ok(())
    }

    fn evaluate_exp(hammer: &mut Hammer, exp: &Vec<Token>) -> Result<(), String>{
        for elt in exp{
            match elt.interp {
                Interp::Operator => hammer.push_txt(&format!("pop r11\npop r10\nmov r12, {}\ncall _operation\npush rax\n", elt.val)),
                Interp::Variable => {
                    let stars = handle_variable_dereference(hammer, &elt)?;
                    match elt.nb_stars{
                        -1 => hammer.push_txt(&format!("push rax\n")),
                        _ => {
                            if stars == 0{
                                let size_def = hammer.get_size_def(elt.val);
                                hammer.push_txt(&format!("{} rax, {}[_stack + rax]\npush rax\n", size_def.mov, size_def.long));
                            }else{
                                hammer.push_txt("movsx rax, dword[_stack + rax]\npush rax\n");
                            }
                        }
                    }
                }
                Interp::Value => hammer.push_txt(&format!("mov rax, {}\npush rax\n", elt.val)),
                Interp::Function => {
                    handle_func_call(hammer, elt.func_dec.as_ref().unwrap().to_string())?;
                }
            }
            
        }
        hammer.push_txt("pop rax\n");
        hammer.replace_txt("push rax\npop rax\n", "");
        Ok(())
    }
    
    fn handle_variable_dereference(hammer: &mut Hammer, var: &Token) -> Result<u32, String> {
        let var_def = hammer.addr_list[&var.val].clone();
        let mut stars = var_def.type_var.stars as i32 - var.nb_stars;
        hammer.push_txt(&format!("mov rbx, {}\nadd rbx, r15\n", var.val));
        for vec in var.squares.as_ref().unwrap().iter() {
            hammer.push_txt("push rbx\n");
            evaluate_exp(hammer, vec)?;
            hammer.push_txt("pop rbx\npush rax\nmov rax, rbx\n_deref 1\nmov rbx, rax\npop rax\nmov rcx, 4\nmul rcx\nadd rbx, rax\n");
            stars -= 1
        }
        hammer.push_txt("mov rax, rbx\n");
        if var.nb_stars > 0 {
            hammer.push_txt(&format!("\n_deref {}\n", var.nb_stars));
        }
        if stars < 0 {
            return Err(format!("Line {}: You tried to dereference the variable {} {} times but you only have the right to dereference it {} times", get_ln(), var_def.name, var_def.type_var.stars as i32 - stars, var_def.type_var.stars));
        }
        Ok(stars as u32)
    }

    fn reset_asm_file() -> Result<(), String>{
        let mut macro_file = TextFile::new(String::from("asm/macros.asm"))?;
        let mut data_file = TextFile::new(String::from("asm/data.asm"))?;
        let mut script_file = TextFile::new(String::from("asm/script.asm"))?;
        let mut func_file = TextFile::new(String::from("asm/functions.asm"))?;

        let mut base_macro_file: TextFile = TextFile::new(String::from("asm/base_files/base_macros.asm"))?;
        let mut base_data_file = TextFile::new(String::from("asm/base_files/base_data.asm"))?;
        let mut base_script_file = TextFile::new(String::from("asm/base_files/base_script.asm"))?;
        let mut base_funcs_file = TextFile::new(String::from("asm/base_files/base_functions.asm"))?;

        macro_file.reset(&base_macro_file.get_text());
        script_file.reset(&base_script_file.get_text());
        data_file.reset(&base_data_file.get_text());
        func_file.reset(&base_funcs_file.get_text());

        Ok(())
    }
    
    fn break_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        no_txt_after_keyword(rest_of_line, "break")?;
        if hammer.loop_index_stack.size() == 0 {
            return Err(format!("Line {}: You can't use the break keyword outside of a loop bloc.", get_ln()));
        }
        hammer.push_txt(&format!("jmp _end_loop_{}\n", hammer.loop_index_stack.val()));
        Ok(())
    }

    fn continue_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        no_txt_after_keyword(rest_of_line, "continue")?;
        if hammer.loop_index_stack.size() == 0 {
            return Err(format!("Line {}: You can't use the continue keyword outside of a loop bloc.", get_ln()));
        }
        hammer.push_txt(&format!("jmp _loop_{}\n", hammer.loop_index_stack.val()));
        Ok(())
    }

    fn return_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        if hammer.txt_stack.size() == 1 {
            return Err(format!("Line {}: You can't use the return keyword outside of a function.", get_ln()));
        }
        no_txt_after_keyword(rest_of_line, "return")?;
        hammer.push_txt("ret\n");
        Ok(())
    }

    fn no_txt_after_keyword(rest_of_line: &String, keyword: &str) -> Result<(), String> {
        if rest_of_line.trim().to_string() != "" {
            return Err(format!("Line {}: we found text after the {} keyword: {}", get_ln(), keyword, rest_of_line));
        } 
        Ok(())
    }

    fn loop_keyword(hammer: &mut Hammer, _rest_of_line: &String) -> Result<(), String> {
        new_loop(hammer);
        end_of_inst(hammer, format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn if_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        evaluate_exp(hammer, &build_aff_vec(hammer, String::from(rest_of_line), MAX_STARS+1)?)?;
        hammer.push_txt(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
        hammer.cond_index_stack.push(hammer.blocs_index);
        end_of_inst(hammer, format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", hammer.blocs_index, hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn else_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        let cond_index = *hammer.cond_index_stack.val();
        let end_txt: String;
        hammer.replace_txt(&format!("_real_end_condition_{}:\n", cond_index), "" );
        if rest_of_line.len() != 0 {
            end_txt = format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", cond_index, hammer.blocs_index, cond_index);
            let first_word = rest_of_line.split(" ").next().unwrap();
            if first_word == "if"{
                evaluate_exp(hammer, &build_aff_vec(hammer, String::from(&rest_of_line[2..rest_of_line.len()]), MAX_STARS+1)?)?;
                hammer.push_txt(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
            } else {
                return Err(format!("Line {}: We found {} when nothing or the if keyword was attempt.", get_ln(), first_word))
            }
        }else{
            end_txt = format!("_real_end_condition_{}:\n", cond_index);
            hammer.cond_index_stack.pop();
        }
        end_of_inst(hammer, end_txt);
        Ok(())
    }

    fn while_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        new_loop(hammer);
        evaluate_exp(hammer, &build_aff_vec(hammer, String::from(rest_of_line), MAX_STARS+1)?)?;
        hammer.push_txt(&format!("cmp rax, 0\nje _end_loop_{}\n", hammer.blocs_index));
        end_of_inst(hammer, format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn new_loop(hammer: &mut Hammer) {
        hammer.push_txt(&format!("_loop_{}:\n", hammer.blocs_index));
        hammer.loop_index_stack.push(hammer.blocs_index);
    }

    fn end_of_inst(hammer: &mut Hammer, end_txt: String) {
        hammer.jumps_stack.push(Jump::new(hammer.stack_index, (end_of_bloc, end_txt), hammer.blocs_index));
        hammer.blocs_index += 1;
    }

    fn end_of_bloc(hammer: &mut Hammer, end_txt: String) {
        hammer.push_txt(&end_txt);
    }

    fn end_of_func(hammer: &mut Hammer, func_name: String) {
        let mut func_file = TextFile::new(String::from("asm/functions.asm")).unwrap();
        func_file.push(&format!("{}:\n{}ret", func_name, hammer.txt_stack.pop()));
        hammer.stack_index = hammer.jumps_stack.val().stack_index;
    }

    fn func_keyword(hammer: &mut Hammer, line: &String) -> Result<(), String> {
        let mut split_par: Vec::<&str> = line.split("(").collect();
        if split_par.len() != 2 {
            return Err(format!("Line {}: Syntax error.", get_ln()))
        }
        let func_name = split_par.remove(0);
        hammer.is_valid_name(func_name)?;
        let line_1 = String::from(split_par[0]);
        split_par = line_1.split(")").collect();
        if split_par.len() != 2 {
            return Err(format!("Line {}: Bad parenthesis.", get_ln()));
        }
        let mut args = Vec::<VariableDefinition>::new();
        for full_elt in split_par[0].split(",") {
            if args.len() == 0 && full_elt == "" {
                break;
            }
            let split_elt: Vec::<String> = full_elt.trim().split(" ").map(String::from).collect();
            if split_elt.len() != 2 {
                return Err(format!("Line {}: We found {} when a variable definition was attempt.", get_ln(), full_elt))
            }
            let name_arg = split_elt[1].clone().trim().to_string();
            hammer.is_valid_name(&name_arg)?;
            args.push(VariableDefinition{
                name: name_arg, 
                type_var: hammer.type_exists(&mut split_elt[0].trim().to_string())?}
            )
        }
        let split_arrow: Vec::<String> = split_par[1].split("->").map(String::from).collect();
        if split_arrow.len() != 2 || split_arrow[0].trim().to_string() != "" {
            return Err(format!("Line {}: Incorrect type return definition for the function {}.", get_ln(), func_name));
        }
        hammer.txt_stack.push(String::new());
        hammer.jumps_stack.push(Jump::new(hammer.stack_index, (end_of_func, String::from(func_name)), hammer.blocs_index));
        hammer.stack_index = 0;
        hammer.blocs_index += 1;
        for elt in &args {
            hammer.define_new_var(elt.name.clone(), elt.type_var.clone());
        }
        hammer.define_new_function(String::from(func_name), args, hammer.type_exists(&mut split_arrow[1].trim().to_string())?)?;
        Ok(())
    }

}

