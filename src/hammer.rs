#[allow(dead_code)]
pub mod hammer{
    use std::collections::HashMap;
    use crate::stack::Stack;
    use crate::tools::tools::*;

    static POINTER: &str = "$";
    static POINTER_SIZE: u32 = 4;
    static MAX_STARS: u32 = 10000; 


    #[derive(Debug)]
    struct Type {
        name: String,
        id: i32,
        size: u32,
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
        vars: Vec::<u32>, 
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


    struct Insertion {
        value: i64,
        lines: Vec<i32>,
        register: String
    }

    struct Tracker{
        registers: Registers,
        stack: Stack<Insertion>,
        line_to_delete: Vec<i32>
    }

    impl Tracker {

        pub fn new() -> Tracker {
            Tracker{
                registers: Registers::new(),
                stack: Stack::new(),
                line_to_delete: Vec::new()
            }
        }

        pub fn new_inst(&mut self, inst: String) -> String {
            let tokens = tokenise_asm_inst(&inst);
            match tokens.len() {
                2 => todo!(),
                _ => {
                    match self.registers.extract_val(&tokens[2]) {
                            Some(val) => {
                            if self.registers.is_followed(&tokens[1]) {
                                match &tokens[0] as &str{
                                    "add" => {
                                        match self.registers.add_val(&tokens[1], val){
                                            Some(()) => return String::new(),
                                            _ => return 
                                        }

                                    }
                                    _ => self.registers.set_val(&tokens[1], val)
                                }
                                return String::new()
                            }
                            _ =>
                        }
                    }
                }
            }
            inst.push('\n');
            inst
        }

        pub fn clean_line(&mut self, txt: &mut String) {
            let mut split_txt: Vec<&str> = txt.split("\n").collect();
            for line_number in self.line_to_delete.iter().rev() {
                split_txt.remove(*line_number as usize);
            }
            self.line_to_delete = Vec::new();
        }
    } 

    fn tokenise_asm_inst(inst: &String) -> Vec<String> {
        todo!()
    }

    struct Registers {
        map: HashMap<&'static str, Option<i64>>,
        convert: HashMap<String, &'static str>
    }

    impl Registers {
        pub fn new() -> Registers{
            let mut res = Registers{
                map: HashMap::<&'static str, Option<i64>>::new(),
                convert: HashMap::<String, &'static str>::new() 
            };
            res.map.insert("rax", None);
            res.map.insert("rbx", None);
            res.convert.insert(String::from("rax"), "rax");
            res.convert.insert(String::from("eax"), "rax");
            res.convert.insert(String::from("ax"), "rax");
            res.convert.insert(String::from("rbx"), "rax");
            res.convert.insert(String::from("ebx"), "rax");
            res.convert.insert(String::from("bx"), "rax");
            res
        }

        pub fn get_val(&self, register: &str) -> Option<i64> {
            self.map.get(self.convert.get(register).unwrap()).unwrap().clone()
        }

        pub fn set_val(&mut self, register: &str, val: i64) {
            *self.map.get_mut(self.convert.get(register).unwrap()).unwrap() = Some(val);
        }

        pub fn add_val(&mut self, register: &str, val: i64) -> Option<()>{
            let previous_val = self.map.get(self.convert.get(register).unwrap()).unwrap();
            match previous_val {
                Some(prev) => return Some(self.set_val(register, val+prev)),
                _ => return None
            }
        }

        pub fn is_followed(&self, register: &str) -> bool {
            self.convert.contains_key(register)
        }


        pub fn extract_val(&self, elt: &str) -> Option<i64> {
            match str::parse::<i64>(elt) {
                Ok(res) => Some(res),
                _ => {
                    if self.is_followed(elt) {
                        return self.get_val(elt)
                    }
                    None
                }
            }
        }
    }

    struct Program {
        inst_vec: Vec<String>,
        txt_stack: Stack<String>,
        loop_index_stack: Stack<u32>,
        cond_index_stack: Stack<u32>,
        type_return: Option<Type>,
        line_number_stack: Stack<(usize, usize)>,
        prog_name: String,
        tracker: Tracker
    }

    impl Program {
        pub fn new(prog_name: String, txt: String) -> Program{
            Program {
                inst_vec: split(&txt, ";"),
                txt_stack: Stack::init(String::new()),
                loop_index_stack: Stack::new(),
                cond_index_stack: Stack::new(),
                type_return: None,
                line_number_stack: Stack::init((1, 0)),
                prog_name: prog_name,
                tracker: Tracker::new()
            }
        }

        fn jump_out(&mut self, bloc_index: u32) {
            if !self.loop_index_stack.is_empty() && *self.loop_index_stack.val() == bloc_index{
                self.loop_index_stack.pop();
            }
        }

        fn inst(&mut self) -> &mut String {
            let inst_number = self.get_in();
            &mut self.inst_vec[inst_number]
        }

        fn push_txt(&mut self, txt: &str) {   
            self.txt_stack.val_mut().push_str(txt);
        }

        fn replace_txt(&mut self, txt1: &str, txt2: &str) {
            let new_txt = self.txt_stack.val_mut().replace(txt1, txt2);
            self.txt_stack.change_top(new_txt); 
        }

        fn get_ln(&self) -> usize {
            self.line_number_stack.val().0   
        }
    
        fn get_in(&self) ->usize {
            self.line_number_stack.val().1
        } 
    
        fn inc_in(&mut self, x: usize) {
            self.line_number_stack.val_mut().1 += x
        }
    
        fn inc_ln(&mut self, x: usize) {
            self.line_number_stack.val_mut().0 += x
        }
    }

    struct Hammer{
        tools: Tools,
        asm_files: HashMap<&'static str, TextFile>,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<u32>>,
        addr_list: HashMap::<u32, Stack::<VariableDefinition>>,
        macro_list: HashMap::<String, u32>,
        keyword_list: HashMap::<String, fn(&mut Hammer, &String) -> Result<(), String>>,
        func_list: HashMap::<String, Function>,
        jumps_stack: Stack<Jump>,
        size: HashMap<u32, AsmType>,
        stack_index: u32,
        blocs_index: u32,
        prog_stack: Stack<Program>,
        file_compiled: Vec<String>,
    }

    impl<'a> Hammer{
        pub fn new(prog_name: String, txt: String)->Hammer{
            let mut res = Hammer{
                tools: Tools::new(),
                asm_files: HashMap::<&'static str, TextFile>::new(),
                type_list: HashMap::<String, Type>::new(),
                defined_var_list: HashMap::<String, Stack<u32>>::new(),
                addr_list: HashMap::<u32, Stack<VariableDefinition>>::new(),
                macro_list: HashMap::<String, u32>::new(),
                keyword_list: HashMap::<String, fn(&mut Hammer, &String) -> Result<(), String>>::new(),
                func_list: HashMap::<String, Function>::new(),
                jumps_stack: Stack::new(),
                size: HashMap::<u32, AsmType>::new(),
                stack_index : 0,
                blocs_index: 1,
                prog_stack: Stack::init(Program::new(prog_name, txt)),
                file_compiled: Vec::new(),
            };
            res.init_asm_file();
            res.init_size();
            res.init_dispo_type();
            res.init_dispo_macro();
            res.init_dispo_keyword();
            res.init();
            return res
        }

        fn compile_new_prog(&mut self, prog_name: String, txt: String) -> Result<(), String> {
            self.prog_stack.push(Program::new(prog_name, txt));
            self.jumps_stack.push(Jump::new(self.stack_index, (end_prog, String::new()), self.blocs_index));
            instruct_loop(self)
        }


        fn init(&mut self) {
            self.jumps_stack.push(Jump::new(0, (end_prog, String::new()), 0));
        }

        fn init_asm_file(&mut self) {
            let paths = vec!("macros", "data", "functions", "script");
            let base_paths = vec!("base_macros", "base_data", "base_functions", "base_script");
            for (i, path) in paths.iter().enumerate() {
                let mut file_path = String::from("asm/");
                let mut base_file_path = String::from("asm/base_files/base_");
                file_path.push_str(path);
                base_file_path.push_str(path);
                file_path.push_str(".asm");
                base_file_path.push_str(".asm");
                let mut file = TextFile::new(file_path).unwrap();
                let mut base_file = TextFile::new(base_file_path).unwrap();
                file.reset(&base_file.get_text());
                self.asm_files.insert(path, file);
                self.asm_files.insert(base_paths[i], base_file);
            }
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

        fn init_dispo_type(&mut self){
            self.new_type(String::from("int"), 4);
            self.new_type(String::from("char"), 1);
            self.new_type(String::from("void"), 0);
        }

        fn new_type(&mut self, name: String, size: u32){
            self.type_list.insert( name.clone(), Type{name: name, id: self.type_list.len() as i32, size: size, stars: 0});   
        }

        fn init_dispo_macro(&mut self){
            self.macro_list.insert(String::from("dn"), 1);
            self.macro_list.insert(String::from("exit"), 1);
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
            self.keyword_list.insert(String::from("for"), for_keyword);
            self.keyword_list.insert(String::from("import"), import_keyword);
        }

        pub fn jump_out(&mut self){
            let last_jump = self.jumps_stack.val();
            self.prog_stack.val_mut().jump_out(last_jump.bloc_index);
            self.stack_index = last_jump.stack_index;
            for addr in &last_jump.vars{
                let addr_stack: &mut Stack<_> = self.addr_list.get_mut(addr).unwrap();
                let def = addr_stack.pop();
                if addr_stack.is_empty() {
                    self.addr_list.remove(addr);
                }
                let var_stack: &mut Stack<_> = self.defined_var_list.get_mut(&def.name).unwrap();
                var_stack.pop();
                if var_stack.is_empty() {
                    self.defined_var_list.remove(&def.name); 
                }
            }
            last_jump.action.0(self, last_jump.action.1.clone()); //We call the function to call at the end of the jump with in parameter the end txt. 
            self.jumps_stack.pop(); 
        }

        fn top_prog_mut(&mut self) -> &mut Program {
            self.prog_stack.val_mut()
        }

        fn top_prog(&self) -> &Program {
            self.prog_stack.val()
        }

        fn inst(&mut self) -> &mut String {
            self.top_prog_mut().inst()
        }

        fn push_txt(&mut self, txt: &str) {   
            self.top_prog_mut().push_txt(txt);
        }

        fn replace_txt(&mut self, txt1: &str, txt2: &str) {
            self.top_prog_mut().replace_txt(txt1, txt2)
        }

        fn define_new_function(&mut self, name: String, args: Vec::<VariableDefinition>, return_type: Type) -> Result<(), String> {
            if self.func_exists(&name) {
                return Err(format!("{} The function {} already exists", self.get_ln(), name))
            }else{
                self.top_prog_mut().type_return = Some(return_type.clone());
                self.func_list.insert(name, Function{args: args, return_type: return_type});
            }
            Ok(())
        }

        fn pop_current_txt(&mut self) -> String {
            self.top_prog_mut().txt_stack.pop()
        }

        fn func_exists(&self, name: &str) -> bool {
            self.func_list.contains_key(name)
        }       
        
        fn error_msg(&self) -> String {
            format!("Error while compiling the file {}. Line {}:", self.top_prog().prog_name, self.get_ln())
        }

        fn type_exists(&self, name: &mut String) -> Result<Type, String>{
            let nb_stars = extract_end_char(name, '*');
            if self.type_list.contains_key(name){
                let mut result = self.type_list[name].clone();
                result.stars = nb_stars;
                return Ok(result)
            }else{
                Err(format!("{} The type {} not exists.", self.get_ln(), name))
            }
        }
        
        fn push_txt_in_file(&mut self, file_name: &str, txt: &mut String) {
            self.top_prog_mut().tracker.clean_line(txt);
            self.asm_files.get_mut(file_name).unwrap().push(txt);
        }

        fn get_ln(&self) -> usize {
            self.top_prog().get_ln()
        }
    
        fn get_in(&self) ->usize {
            self.top_prog().get_in()
        } 
    
        fn inc_in(&mut self, x: usize) {
            self.top_prog_mut().inc_in(x)
        }
    
        fn inc_ln(&mut self, x: usize) {
            self.top_prog_mut().inc_ln(x)
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

        pub fn get_addr(&self, name: &str) -> u32{
            *self.defined_var_list[name].val()
        }

        pub fn get_var_def_by_name(&self, name: &str) -> &VariableDefinition{
            &self.addr_list[&self.get_addr(name)].val()
        }

        pub fn get_var_def_i32(&self, addr: i32) -> &VariableDefinition {
            self.get_var_def(addr as u32)
        }

        pub fn get_var_def(&self, addr: u32) -> &VariableDefinition {
            &self.addr_list[&addr].val()
        }

        pub fn define_new_var(&mut self, name: String, type_var: Type){
            let addr = self.stack_index;
            self.stack_index += type_var.size;
            let var_def = VariableDefinition{
                name: name.clone(),
                type_var: type_var
            };
            match self.addr_list.get_mut(&addr) {
                Some(stack) => stack.push(var_def),
                _ => {self.addr_list.insert(addr, Stack::init(var_def));}
            }
            self.jumps_stack.val_mut().vars.push(addr);
            if !self.var_exists(&name){
                self.defined_var_list.insert(name, Stack::<u32>::init(addr));    
            }else{
                self.defined_var_list.get_mut(&name).unwrap().push(addr)
            }
        }

        
        pub fn get_size_def(&self, addr: u32) -> &AsmType{
            &self.size[&(self.addr_list[&addr].val().type_var.size as u32)]
        }

        pub fn is_valid_name(&self, name: &str) -> Result<(), String>{
            if name != "" && name.chars().nth(0).unwrap() == '_'{
                return Err(String::from("A name can't start with '_' it's a vulcain private use."));
            }else if !self.tools.is_valid_name(name){
                return Err(format!("{} is not a valid name for a variable.", name))
            }
            Ok(())
        }

        pub fn txt_stack(&mut self) -> &mut Stack<String> {
            &mut self.top_prog_mut().txt_stack
        }

        pub fn loop_index_stack(&mut self) -> &mut Stack<u32> {
            &mut self.top_prog_mut().loop_index_stack
        }

        pub fn cond_index_stack(&mut self) -> &mut Stack<u32> {
            &mut self.top_prog_mut().cond_index_stack
        }

        pub fn in_func(&self) -> bool {
            self.top_prog().type_return.is_some()
        }

        pub fn type_return(&mut self) -> &mut Option<Type> {
            &mut self.top_prog_mut().type_return
        }

        pub fn reset_type_return(&mut self) {
            self.top_prog_mut().type_return = None
        }

    }

    pub fn compile_txt(prog_name: String, input: String) -> Result<(), String>{
        let mut hammer: Hammer = Hammer::new(prog_name, input);
        instruct_loop(&mut hammer)?;
        Ok(())
    }

    fn instruct_loop(hammer: &mut Hammer) -> Result<(), String>{
        while hammer.get_in() <= hammer.top_prog().inst_vec.len() -1{
            let nb_back_line = hammer.inst().matches('\n').count();
            hammer.inc_ln(nb_back_line);
            *hammer.inst() = hammer.inst().replace("\n", "").trim().to_string();
            if hammer.inst().starts_with("}") {
                hammer.inst().remove(0);
                hammer.jump_out();
            } else if hammer.inst().contains("{") {
                let mut split_inst: Vec::<String> = split(hammer.inst() as &str, "{");
                let inst_behind = split_inst.remove(0).trim().to_string();
                *hammer.inst() = annalyse_inst_behind_bracket(hammer, inst_behind, split_inst.join("{"))?;
            }else{
                setup_inst(hammer.inst());
                let inst = hammer.inst().to_string();
                handle_instruction(hammer, inst)?;
                hammer.inc_in(1);
            }
        }
        hammer.jump_out();
        Ok(())
    }

    fn handle_instruction(hammer: &mut Hammer, mut inst: String) -> Result<(), String> {
        let mut line_split = split(&inst, " ");
        if line_split.len() != 0{
            match hammer.type_exists(&mut line_split[0]) {
                Ok(mut type_var) => {
                    dec_new_var(hammer, &line_split, &mut type_var)?;
                    if inst.contains("="){
                        line_split.remove(0);
                        handle_affectation(hammer, line_split.join(" "))?;
                    }
                }   
                _ => {
                    if hammer.func_exists(line_split[0].split("(").next().unwrap()) {
                        handle_func_call(hammer, inst)?;
                    }else if inst.starts_with("!"){
                        inst.remove(0);
                        handle_macros_call(hammer, inst)?; 
                    }else if hammer.keyword_exists(&line_split[0]){
                        hammer.call_keyword(&line_split.remove(0), &line_split.join(" "))?;
                    }else if inst.contains("="){
                        handle_affectation(hammer, inst)?;
                    }else if inst != ""{
                        return Err(format!("{} Syntax error", hammer.error_msg()));
                    }
                }
            }
        } 
        Ok(())
    }


    fn handle_func_call(hammer: &mut Hammer, mut call: String) -> Result<(), String> {
        let mut split_par: Vec::<String> = call.split("(").map(String::from).collect();
        if split_par.len() == 1 {
            return Err(format!("{} You have to specifie args between parenthesis.", hammer.error_msg()))
        }
        let func_name = split_par.remove(0);
        let func = &hammer.func_list[&func_name].clone();
        call = split_par.join("(");
        if call.chars().rev().nth(0).unwrap() != ')' {
            return Err(format!("{} Parenthesis never closes.", hammer.error_msg()));
        }
        call = String::from(&call[0..call.len()-1]);
        let mut split_virg: Vec::<String> = call.split(",").map(String::from).collect();
        if split_virg.len() == 1 && split_virg[0].trim().to_string() == "" {
            split_virg.remove(0);
        }
        if split_virg.len() != func.args.len() {
            return Err(format!("{} We found {} elements in the call of the function {} but {} takes {} arguments.", hammer.error_msg(), split_virg.len(), func_name, func_name, func.args.len()));
        }
        let mut decal = 0;
        for (i, exp) in split_virg.iter().enumerate() {
            put_res_in_rax(hammer, exp.to_string(), func.args[i].type_var.stars)?;
            hammer.push_txt(&format!("mov [_stack+r15+ {}], {}\n", hammer.stack_index + decal, hammer.size[&func.args[i].type_var.size].register));
            decal += func.args[i].type_var.size;
        }
        hammer.push_txt(&format!("push r15\nadd r15, {}\ncall {}\npop r15\n", hammer.stack_index, func_name));
        Ok(())
    }

    fn handle_macros_call(hammer: &mut Hammer, mut line: String) -> Result<(), String>{
        if line.pop() != Some(')'){
            return Err(format!("{} Syntax error.", hammer.error_msg()));   
        }
        let mut split_par = line.split("(");
        let mut name = String::from(split_par.next().unwrap());
        name = name.trim().to_string();
        if !hammer.macro_exists(&name){
            return Err(format!("{} The macro {} doesn't exists.", hammer.error_msg(), name));
        }
        line = match split_par.next() {
            Some(rest_of_line)=> String::from(rest_of_line),
            _ => return Err(format!("{} Parenthesis missing.", hammer.error_msg()))
        };
        let mut args = Vec::<Vec::<Token>>::new();
        line = line.replace(" ", "");
        for arg in line.split(","){
            if arg == ""{
                break;
            }
            args.push(build_aff_vec(hammer, String::from(arg), MAX_STARS+1)?);
        }
        let nb_arg_expected = hammer.macro_list[&name];
        if args.len() != nb_arg_expected as usize{
            return Err(format!("{} Found {} arguments when {} was expected", hammer.error_msg(), args.len(), nb_arg_expected));
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
            return Err(format!("{} Invalid syntax.", hammer.error_msg()));
        }
        let mut var1 = split[0].trim().to_string();
        split.remove(0);
        let nb_stars = get_prof_pointer(hammer, &mut var1, false)?;
        let tab_vec = tab_analyse(hammer, &mut var1)?;
        if hammer.var_exists(&var1){
            let right_exp = split.join("=").replace(" = ", "=");
            let addr = hammer.get_addr(&var1);
            let struct_addr = Token{val: addr as i32, func_dec: None, squares: Some(tab_vec), nb_stars: nb_stars, interp: Interp::Variable};
            let stars_in_left_var = handle_variable_dereference(hammer, &struct_addr)?;
            hammer.push_txt("push rax\n");
            put_res_in_rax(hammer, right_exp, stars_in_left_var as u32)?;
            if stars_in_left_var != 0 {
                hammer.push_txt("pop rbx\nmov dword[_stack+ rbx], eax\n");
            }else{
                hammer.push_txt(&format!("pop rbx\nmov [_stack+ rbx], {}\n", hammer.get_size_def(addr).register));
            }
            
            Ok(())
        }else {
            return Err(format!("{} The variable {} doesn't exists.", hammer.error_msg(), var1));
        }
       
    }

    fn end_prog(hammer: &mut Hammer, _s: String) {
        let mut txt = hammer.pop_current_txt();
        hammer.push_txt_in_file("script", &mut txt);
        hammer.prog_stack.pop();
    }
    
   
    fn setup_inst(inst: &mut String){
        let mut prev = ' ';
        for (i, chara) in inst.clone().chars().enumerate() {
            if chara == '=' {
                inst.insert(i, ' ');
                if "*+-/%".contains(prev) {
                    inst.remove(i-1);
                    inst.insert_str(i+1, &format!(" {} {}", &inst[0..i], prev));
                }else{
                    inst.insert(i+2, ' ');
                }
                break;
            }
            prev = chara;
        }
        *inst = inst.trim().to_string();
    }

    fn annalyse_inst_behind_bracket(hammer: &mut Hammer, inst: String, rest_of_inst: String) -> Result<String, String> {
        let first_word = inst.split(" ").next().unwrap();
        if hammer.keyword_exists(first_word) {
            hammer.keyword_list[first_word](hammer, &String::from(&inst[first_word.len()..inst.len()]).trim().to_string())?; // We call the keyword function with the rest of the line in parameter
        }else if inst == "" {
            jump_in(hammer, String::from(""));
        }else if inst.ends_with("="){
            direct_dec_tab(hammer, inst, rest_of_inst)?;         
            return Ok(String::new())   
        }else {
            return Err(format!("{} Syntax error, we found {} behind brackets.", hammer.error_msg(), inst))
        }
        Ok(rest_of_inst)
    }


    fn direct_dec_tab(hammer: &mut Hammer, inst: String, rest_of_inst: String) -> Result<(), String> {
        let mut inst_split: Vec<String> = split(&inst, " ");
        match hammer.type_exists(&mut inst_split[0]) {
            Ok(mut type_var) => {
                let data_square = dec_new_var(hammer, &inst_split, &mut type_var)?;
                let s = type_var.size;
                let mut arrays: Vec<String> = split(&rest_of_inst, "{");
                let mut i = 0;
                for arr in arrays.iter_mut(){
                    *arr = arr.trim().to_string();
                    if arr != ""{
                        if arr.pop() != Some('}') && arr.pop() != Some('}') {
                            return Err(format!("{} You forgot to close a bracket.", hammer.error_msg()));
                        }
                        while arr.chars().last() == Some('}') || arr.chars().last() == Some(' '){
                            arr.pop();
                        }
                        let val_vec: Vec<&str> = arr.split(",").collect();
                        if val_vec.len() > data_square.0 as usize {
                            return Err(format!("{} You put {} values between bracket when {} at maximum was attempt.", hammer.error_msg(), val_vec.len(), data_square.0));
                        }
                        for j in 0..val_vec.len() as u32{
                            put_res_in_rax(hammer, val_vec[j as usize].trim().to_string(), type_var.stars)?;
                            hammer.push_txt(&format!("mov [_stack+r15+ {}], {}\n", i*s*data_square.0 + data_square.1 + s*j, hammer.size[&s].register));
                        }
                        i += 1;
                    }
                }
            }   
            _ => return Err(format!("{} Syntax error, we found {} behind brackets.", hammer.error_msg(), inst))
        }
        Ok(())
    }

    fn dec_new_var(hammer: &mut Hammer, inst_split: &Vec<String>, type_var: &mut Type) -> Result<(u32, u32), String> {
        if inst_split.len() < 2{
            return Err(format!("{} You should initialise a variable.", hammer.error_msg()))
        }
        let mut split_var: Vec::<&str> = inst_split[1].split("[").collect();
        hammer.is_valid_name(split_var[0])?;
        let data_square = handle_tab_type_dec(hammer, &mut split_var, type_var)?;
        hammer.define_new_var(String::from(split_var[0]), type_var.clone());
        Ok(data_square)
    }

    fn put_res_in_rax(hammer: &mut Hammer, exp: String, nb_stars_await: u32) -> Result<(), String> {
        evaluate_exp(hammer, &build_aff_vec(hammer, exp, nb_stars_await)?)
    }

    fn handle_tab_type_dec(hammer: &mut Hammer, var: &mut Vec<&str>, type_var: &mut Type) -> Result<(u32, u32), String> {
        let mut previous_data: (u32, u32) = (1, hammer.stack_index);
        if var.len() != 1 {
            let tab_addr = hammer.stack_index;
            for i in 1..var.len() {
                if var[i] == "" || var[i].chars().last().unwrap() != ']'{
                    return Err(format!("{} You forgot to close the bracket with '['", hammer.error_msg()))
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
                            hammer.push_txt(&format!("mov {}[_stack+r15+ {}], {}\n", hammer.size[&size].long, previous_data.1+size*j, hammer.stack_index));
                            hammer.stack_index += size*tab_size;
                        }
                        if i != var.len()-1 {
                            previous_data.0 *= tab_size;
                        }else{
                            previous_data.0 = tab_size;
                        }
                    },
                    Err(_) => return Err(format!("{} You didn't write a correct number between the brackets", hammer.error_msg()))
                }
                previous_data.1 = stack_index;
            }
            hammer.push_txt(&format!("mov rdx, {}\nadd rdx, r15\nmov rax, {}\nadd rax, r15\nmov [_stack+ rax], edx\n", tab_addr, hammer.stack_index));
        }
        Ok(previous_data)
    }


    fn tab_analyse(hammer: &Hammer, var_s: &mut String) -> Result<Vec::<Vec<Token>>, String>{
        let var: Vec::<&str> = var_s.split("[").collect();
        let mut res = Vec::<Vec<Token>>::new();
        for i in 1..var.len() {
            if var[i] == "" || var[i].chars().last().unwrap() != ']'{
                return Err(format!("{} You forgot to close with '['", hammer.error_msg()))
            }
            res.push(build_aff_vec(hammer, String::from(&var[i][0..var[i].len()-1]), 0)?);
        }
        *var_s = String::from(var[0]);
        Ok(res)
    }
    

    fn get_prof_pointer(hammer: &Hammer, var: &mut String, can_be_ref: bool) -> Result<i32, String>{
        let mut count: i32 = 0;
        while var.len() != 0 && (var.starts_with(POINTER) || var.starts_with("&")){
            count += var.starts_with(POINTER) as i32 - !var.starts_with(POINTER) as i32;
            if count == -2{
                return Err(format!("{} You tried to get the adress of a direct value.", hammer.error_msg()));
            }
            var.remove(0);
        }
        if count == -1 && !can_be_ref {
            return Err(format!("{} You can't put a reference here.", hammer.error_msg()));
        }
        Ok(count)
    }

    fn format_string_exp(hammer: &Hammer, mut string_exp: String) -> String {
        for op in hammer.tools.get_op_iter(){
            string_exp = string_exp.replace(op, &format!("µ{}µ", op));
        }
        string_exp = string_exp.replace("(", "µ(µ");
        string_exp = string_exp.replace(")", "µ)µ");
        while string_exp.contains("µµ") {
            string_exp = string_exp.replace("µµ", "µ");
        }
        string_exp = string_exp.replace("<µ=", "<=µ");
        string_exp = string_exp.replace(">µ=", ">=µ");
        string_exp
    }

    fn build_aff_vec(hammer: &Hammer, mut string_exp: String, nb_stars_await: u32) -> Result<Vec::<Token>, String>{
        string_exp = string_exp.trim().to_string();
        if string_exp == String::from("") || string_exp.contains("_") && is_in_a_string(&string_exp, "_".to_string()){
            return Err(format!("{} Syntax error.", hammer.error_msg()));
        }
        string_exp = format_string_exp(hammer, string_exp);
        let exp: Vec::<String>  = string_exp.split("µ").map(String::from).collect();
        let mut exp_res = tokenize_expression(hammer, exp)?;
        exp_res = hammer.tools.convert_in_postfix_exp(exp_res);
        let mut res = Vec::<Token>::new();
        for elt in exp_res.iter(){
            add_element_in_aff_exp(hammer, elt.to_string(), &mut res, nb_stars_await)?;
        }
        Ok(res)
    }

    fn tokenize_expression(hammer: &Hammer, exp: Vec::<String>) -> Result<Vec<String>, String> {
        let mut exp_res = Vec::<String>::new();
        let mut func_dec = String::new();
        let mut cant_be_op = true;
        let mut neg_count: u32 = 0;
        let mut looking_for_function = -1;
        let mut par_count = 0;
        for i in 0..exp.len(){
            let word = exp[i].trim().to_string();
            if word != "" {
                par_count += (word == "(") as i32 - (word == ")") as i32; 
                if looking_for_function == -1 {
                    let is_op = hammer.tools.is_operator(&word); 
                    if is_op || hammer.tools.is_separator(&word){
                        neg_count *= (is_op && cant_be_op) as u32; 
                        if is_op {
                            if cant_be_op{
                                match &word as &str{
                                    "-" => neg_count += 1,
                                    _ => return Err(format!("{} There is a bad operator in your expression.", hammer.error_msg()))
                                }   
                            }else{
                                exp_res.push(word);
                                cant_be_op = true;
                            }
                        }else {

                            exp_res.push(word);
                            cant_be_op = false;
                        }
                    }else{
                        if hammer.func_exists(&word) {
                            func_dec = word;
                            looking_for_function = par_count;
                        }else{
                            if neg_count % 2 == 0 {
                                exp_res.push(word);
                            }else{
                                exp_res.push(String::from("-1"));
                                exp_res.push(String::from("*"));
                                exp_res.push(word);
                            }
                        }
                        cant_be_op = false;
                    }
                }else{
                    func_dec.push_str(&word);
                    if looking_for_function == par_count {
                        exp_res.push(func_dec);
                        func_dec = String::new();
                        looking_for_function = -1;
                    }                                       
                }
            }
        }
        if par_count != 0 {
            return Err(format!("{} The number of opening brackets is not the same as the number of closing brackets", hammer.error_msg()));
        }
        Ok(exp_res)
    }


    fn is_in_a_string(_exp: &String, _part: String) -> bool {
        false
    }

    fn add_element_in_aff_exp(hammer: &Hammer, mut element: String, exp: &mut Vec::<Token>, nb_stars_await: u32) -> Result<(), String>{
        if element == ""{
            return Err(format!("{} Syntax error.", hammer.error_msg()));
        }
        if hammer.tools.is_operator(&element){ 
            exp.push(Token::new_op(hammer.tools.ascii_val(&element)))
        }else{
            match element.parse::<i32>(){
                Ok(number) => exp.push(Token::new_val(number)),
                Err(_e) => {
                    let mut nb_stars = get_prof_pointer(hammer, &mut element, true)?;
                    let var_name = element.split("[").next().unwrap();
                    if hammer.var_exists(&var_name){
                        let var_def = hammer.get_var_def_by_name(&var_name);
                        if (var_def.type_var.stars as i32) < nb_stars {
                            return Err(format!("{} You tried to dereference the variable {} {} times but you only have the right to dereference it {} times", hammer.error_msg(), var_def.name, nb_stars, var_def.type_var.stars));
                        }
                        let tab_vec = tab_analyse(hammer, &mut element)?;
                        nb_stars += tab_vec.len() as i32;
                        if nb_stars_await != MAX_STARS+1 && var_def.type_var.stars as i32 - nb_stars != nb_stars_await as i32{
                            return Err(format!("{} The two types are incompatibles.", hammer.error_msg()));
                        }
                        nb_stars -= tab_vec.len() as i32;
                        exp.push(Token{val: hammer.get_addr(&element) as i32, squares: Some(tab_vec), func_dec: None, nb_stars: nb_stars, interp: Interp::Variable});
                    }else if hammer.func_exists(element.split("(").next().unwrap()){
                        let return_type = &hammer.func_list[element.split("(").next().unwrap()].return_type;
                        if return_type.stars == nb_stars_await && nb_stars_await != MAX_STARS+1 && return_type.name != "void" {
                            exp.push(Token::new_func(element));
                        }else{
                            return Err(format!("{} The two types are incompatibles.", hammer.error_msg()));
                        }
                    }else{
                        match from_char_to_number(&element){
                            Some(val) => {
                                if nb_stars != 0 {
                                    return Err(format!("{} You tried to dereference a character", hammer.error_msg()))
                                }
                                exp.push(Token::new_val(val as i32))
                            },
                            _ => {
                                return Err(format!("{} we found an incorrect value: {}", hammer.error_msg(), element))
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn insert_macro_call_in_txt(hammer: &mut Hammer, macro_call: MacroCall) -> Result<(), String> {
        let mut i = 0;
        for arg in &macro_call.args{
            evaluate_exp(hammer, arg)?;
            hammer.push_txt(&format!("mov [_stack+r15+ {}], rax\n", hammer.stack_index + i*8));
            i += 1;
        }
        hammer.push_txt(&format!("{} ", macro_call.macro_name));
        for j in 0..i {
            hammer.push_txt(&format!("[_stack+r15+ {}] ", hammer.stack_index + j*8));
        }
        hammer.push_txt("\n");
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
                                let size_def = hammer.get_size_def(elt.val as u32);
                                hammer.push_txt(&format!("{} rax, {}[_stack+ rax]\npush rax\n", size_def.mov, size_def.long));
                            }else{
                                hammer.push_txt("movsx rax, dword[_stack+ rax]\npush rax\n");
                            }
                        }
                    }
                }
                Interp::Value => hammer.push_txt(&format!("mov rax, {}\npush rax\n", elt.val)),
                Interp::Function => {
                    handle_func_call(hammer, elt.func_dec.as_ref().unwrap().to_string())?;
                    hammer.push_txt("push rax\n");
                }
            }
            
        }
        hammer.push_txt("pop rax\n");
        hammer.replace_txt("push rax\npop rax\n", "");
        hammer.replace_txt("push rax\npop r11\n", "mov r11, rax\n");
        Ok(())
    }
    
    fn handle_variable_dereference(hammer: &mut Hammer, var: &Token) -> Result<u32, String> {
        let var_def = hammer.get_var_def_i32(var.val).clone();
        let mut stars = var_def.type_var.stars as i32 - var.nb_stars;
        hammer.push_txt(&format!("mov rbx, {}\nadd rbx, r15\n", var.val));
        for vec in var.squares.as_ref().unwrap().iter() {
            hammer.push_txt("push rbx\n");
            evaluate_exp(hammer, vec)?;
            hammer.push_txt("pop rbx\n_deref 1\nmov rcx, 4\nmul rcx\nadd rbx, rax\n");
            stars -= 1
        }
        if var.nb_stars > 0 {
            hammer.push_txt(&format!("\n_deref {}\n", var.nb_stars));
        }
        hammer.push_txt("mov rax, rbx\n");
        if stars < 0 {
            return Err(format!("{} You tried to dereference the variable {} {} times but you only have the right to dereference it {} times", hammer.error_msg(), var_def.name, var_def.type_var.stars as i32 - stars, var_def.type_var.stars));
        }
        is_valid_address(hammer);
        Ok(stars as u32)
    }

    fn is_valid_address(hammer: &mut Hammer) {
        hammer.push_txt(&format!("mov rdx, r15\nadd rdx, {}\ncmp rax, rdx\njg _invalid_address\n", hammer.stack_index));
    }
    
    fn break_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        no_txt_after_keyword(hammer, rest_of_line, "break")?;
        if hammer.loop_index_stack().size() == 0 {
            return Err(format!("{} You can't use the break keyword outside of a loop bloc.", hammer.error_msg()));
        }
        let loop_index = *hammer.loop_index_stack().val();
        hammer.push_txt(&format!("jmp _end_loop_{}\n", loop_index));
        Ok(())
    }

    fn continue_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        no_txt_after_keyword(hammer, rest_of_line, "continue")?;
        if hammer.loop_index_stack().size() == 0 {
            return Err(format!("{} You can't use the continue keyword outside of a loop bloc.", hammer.error_msg()));
        }
        let loop_index = *hammer.loop_index_stack().val();
        hammer.push_txt(&format!("jmp _loop_{}\n", loop_index));
        Ok(())
    }

    fn return_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        if hammer.txt_stack().size() == 1 {
            return Err(format!("{} You can't use the return keyword outside of a function.", hammer.error_msg()));
        }
        let nb_stars_await = hammer.type_return().as_mut().unwrap().stars;
        put_res_in_rax(hammer, rest_of_line.to_string(), nb_stars_await)?;
        hammer.push_txt("ret\n");
        Ok(())
    }

    fn no_txt_after_keyword(hammer: &Hammer, rest_of_line: &String, keyword: &str) -> Result<(), String> {
        if rest_of_line.trim().to_string() != "" {
            return Err(format!("{} we found text after the {} keyword: {}", hammer.error_msg(), keyword, rest_of_line));
        } 
        Ok(())
    }

    fn loop_keyword(hammer: &mut Hammer, _rest_of_line: &String) -> Result<(), String> {
        new_loop(hammer, hammer.blocs_index);
        jump_in(hammer, format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn if_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        put_res_in_rax(hammer, String::from(rest_of_line), MAX_STARS+1)?;
        hammer.push_txt(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
        let bloc_index = hammer.blocs_index;
        hammer.cond_index_stack().push(bloc_index);
        jump_in(hammer, format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", hammer.blocs_index, hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn else_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        let cond_index = *hammer.cond_index_stack().val();
        let end_txt: String;
        hammer.replace_txt(&format!("_real_end_condition_{}:\n", cond_index), "" );
        if rest_of_line.len() != 0 {
            end_txt = format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", cond_index, hammer.blocs_index, cond_index);
            let first_word = rest_of_line.split(" ").next().unwrap();
            if first_word == "if"{
                put_res_in_rax(hammer, String::from(&rest_of_line[2..rest_of_line.len()]), MAX_STARS+1)?;
                hammer.push_txt(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
            } else {
                return Err(format!("{} We found {} when nothing or the if keyword was attempt.", hammer.error_msg(), first_word))
            }
        }else{
            end_txt = format!("_real_end_condition_{}:\n", cond_index);
            hammer.cond_index_stack().pop();
        }
        jump_in(hammer, end_txt);
        Ok(())
    }

    fn while_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String>{
        new_loop(hammer, hammer.blocs_index);
        put_res_in_rax(hammer, String::from(rest_of_line), MAX_STARS+1)?;
        hammer.push_txt(&format!("cmp rax, 0\nje _end_loop_{}\n", hammer.blocs_index));
        jump_in(hammer, format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index));
        Ok(())
    }

    fn for_keyword(hammer: &mut Hammer, rest_of_line: &String) -> Result<(), String> {
        let mut split_exp: Vec::<String> = rest_of_line.split(":").map(String::from).collect();
        if split_exp.len() != 3 {
            return Err(format!("{} Bad declaration of the for loop, we found {} instructions instead of 3.", hammer.error_msg(), split_exp.len()))
        }
        setup_inst(&mut split_exp[0]);
        setup_inst(&mut split_exp[2]);

        jump_in(hammer, format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index));
        handle_instruction(hammer, split_exp[0].clone())?;
        hammer.push_txt(&format!("jmp _loop_{}_end_start_inst\n", hammer.blocs_index-1));
        new_loop(hammer, hammer.blocs_index - 1);
        handle_instruction(hammer, split_exp[2].to_string())?;
        hammer.push_txt(&format!("_loop_{}_end_start_inst:\n", hammer.blocs_index-1));
        put_res_in_rax(hammer, split_exp[1].clone(), MAX_STARS+1)?;
        hammer.push_txt(&format!("cmp rax, 0\nje _end_loop_{}\n", hammer.blocs_index-1));
        Ok(())
    }

    fn new_loop(hammer: &mut Hammer, loop_index: u32) {
        hammer.push_txt(&format!("_loop_{}:\n", loop_index));
        hammer.loop_index_stack().push(loop_index);
    }

    fn jump_in(hammer: &mut Hammer, end_txt: String) {
        hammer.jumps_stack.push(Jump::new(hammer.stack_index, (end_of_bloc, end_txt), hammer.blocs_index));
        hammer.blocs_index += 1;
    }

    fn end_of_bloc(hammer: &mut Hammer, end_txt: String) {
        hammer.push_txt(&end_txt);
    }

    fn end_of_func(hammer: &mut Hammer, func_name: String) {
        let txt = hammer.pop_current_txt();
        hammer.push_txt_in_file("functions", &mut format!("{}:\n{}ret\n", func_name, txt));
        hammer.stack_index = hammer.jumps_stack.val().stack_index;
        hammer.reset_type_return();
    }

    fn func_keyword(hammer: &mut Hammer, line: &String) -> Result<(), String> {
        if hammer.in_func() {
            return Err(format!("{} You can't define a function in a function.", hammer.error_msg()));
        }
        let mut split_par: Vec::<&str> = line.split("(").collect();
        if split_par.len() != 2 {
            return Err(format!("{} Syntax error.", hammer.error_msg()))
        }
        let func_name = split_par.remove(0);
        hammer.is_valid_name(func_name)?;
        let line_1 = String::from(split_par[0]);
        split_par = line_1.split(")").collect();
        if split_par.len() != 2 {
            return Err(format!("{} Bad parenthesis.", hammer.error_msg()));
        }
        let mut args = Vec::<VariableDefinition>::new();
        for full_elt in split_par[0].split(",") {
            if args.len() == 0 && full_elt == "" {
                break;
            }
            let split_elt: Vec::<String> = full_elt.trim().split(" ").map(String::from).collect();
            if split_elt.len() != 2 {
                return Err(format!("{} We found {} when a variable definition was attempt.", hammer.error_msg(), full_elt))
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
            return Err(format!("{} Incorrect type return definition for the function {}.", hammer.error_msg(), func_name));
        }
        hammer.txt_stack().push(String::new());
        hammer.jumps_stack.push(Jump::new(hammer.stack_index, (end_of_func, String::from(func_name)), hammer.blocs_index));
        hammer.stack_index = 0;
        hammer.blocs_index += 1;
        for elt in &args {
            hammer.define_new_var(elt.name.clone(), elt.type_var.clone());
        }
        hammer.define_new_function(String::from(func_name), args, hammer.type_exists(&mut split_arrow[1].trim().to_string())?)?;
        Ok(())
    }

    fn import_keyword(hammer: &mut Hammer, file: &String) -> Result<(), String> {
        if file.ends_with(".vu") {
            if file_exists(file) {
                if !hammer.file_compiled.contains(file){
                    hammer.file_compiled.push(file.to_string());
                    let mut txt = hammer.top_prog_mut().txt_stack.pop();
                    hammer.push_txt_in_file("script", &mut txt);
                    hammer.top_prog_mut().txt_stack.push(String::new());
                    let mut imported_file = TextFile::new(file.to_string())?;
                    hammer.compile_new_prog(file.to_string(), imported_file.get_text())?;
                }
            }else{
                return Err(format!("{} The file {} not exists.", hammer.error_msg(), file));
            }
        }else{
            return Err(format!("{} You only can import .vu files.", hammer.error_msg()))
        }
        Ok(())
    }

}