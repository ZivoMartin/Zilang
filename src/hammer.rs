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

    enum Interp {
        NUMBER,
        VARIABLE,
        CHARACTER
    }


    struct AsmType{
        long: &'static str,
        short: &'static str,
        register: &'static str,
        mov: &'static str
    }

    struct VariableDefinition{
        name: String,
        type_var: Type,
    }
    

    struct MacroCall{
        macro_name: String,
        args: Vec::<Vec::<(Adress, u8)>>
    }

    struct Jump{
        vars: Vec::<i32>, 
        stack_index: i32,
        end_txt: String,
        bloc_index: u32
    }

    impl Jump {

        fn new(stack_index: i32, end_txt: String, bloc_index: u32) -> Self {
            Jump{
                vars: Vec::new(),
                stack_index: stack_index,
                end_txt: end_txt,
                bloc_index: bloc_index
            }
        }

    }
    
    #[derive(Debug)]
    struct Adress {
        val: i32,
        nb_stars: i32,
        squares: Option<Vec::<Vec<(Adress, u8)>>>
    }
    
    impl Adress {

        pub fn new(val: i32) -> Adress{
            Adress{val: val, squares: None, nb_stars: 0}
        }

    }

    struct Hammer{
        tools: Tools,
        type_list: HashMap::<String, Type>,
        defined_var_list: HashMap::<String, Stack::<i32>>,
        addr_list: HashMap::<i32, VariableDefinition>,
        macro_list: HashMap::<String, u32>,
        keyword_list: HashMap::<String, fn(&mut Hammer, &String) -> Result<(), String>>,
        jumps_stack: Stack<Jump>,
        size: HashMap<u32, AsmType>,
        stack_index: u32,
        txt_result: String,
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
                jumps_stack: Stack::new(),
                size: HashMap::<u32, AsmType>::new(),
                stack_index : 0,
                txt_result: String::new(),
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
        }

        pub fn jump_out(&mut self){
            let top_stack = self.jumps_stack.pop();
            if !self.loop_index_stack.is_empty() && *self.loop_index_stack.val() == top_stack.bloc_index as u32{
                self.loop_index_stack.pop();
            }
            self.stack_index = top_stack.stack_index as u32;
            let variable_to_destroy: Vec::<i32> = top_stack.vars; 
            for addr in variable_to_destroy{
                let def = self.addr_list.remove(&addr).unwrap();
                let var_stack: &mut Stack<_> = self.defined_var_list.get_mut(&def.name).unwrap(); 
                var_stack.pop();
                if var_stack.is_empty() {
                    self.defined_var_list.remove(&def.name);
                }
            }
            self.txt_result.push_str(&top_stack.end_txt);
        }


        fn init_dispo_macro(&mut self){
            self.macro_list.insert(
                String::from("dn"), 
                1
            );
            self.macro_list.insert(
                String::from("exit"), 
                1
            );
        }

        fn init_dispo_type(&mut self){
            self.new_type(String::from("int"), 4);
            self.new_type(String::from("char"), 1);
        } 

        fn new_type(&mut self, name: String, size: i32){
            self.type_list.insert( name.clone(), Type{name: name, id: self.type_list.len() as i32, size: size, stars: 0});   
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

        fn get_extract_string(&self, addr: &Adress) -> String{
            return String::from(format!("{}[_stack + {}]", self.get_size_def(addr.val).long, addr.val))
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
        script_file.push(&hammer.txt_result);
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

    
    fn instruct_loop(hammer: &mut Hammer, vec: &mut Vec::<String>) -> Result<(), String>{
        inc_in(1);
        hammer.jumps_stack.push(Jump::new(0, String::new(), 0));

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
    }

    fn annalyse_inst_behind_bracket(hammer: &mut Hammer, mut inst: String) -> Result<(), String> {
        let mut end_txt = String::new();
        if inst.starts_with("if"){
            evaluate_exp(hammer, &build_aff_vec(hammer, String::from(&inst[2..inst.len()]), MAX_STARS+1)?);
            hammer.txt_result.push_str(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
            end_txt = format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", hammer.blocs_index, hammer.blocs_index, hammer.blocs_index);
            hammer.cond_index_stack.push(hammer.blocs_index);
        }else if inst.starts_with("else"){
            let cond_index = *hammer.cond_index_stack.val();
            hammer.txt_result = hammer.txt_result.replace(&format!("_real_end_condition_{}:\n", cond_index), "" );
            if inst.len() != 4 {
                inst = String::from(&inst[5..inst.len()]).trim().to_string();
                end_txt = format!("jmp _real_end_condition_{}\n_end_condition_{}:\n_real_end_condition_{}:\n", cond_index, hammer.blocs_index, cond_index);
                evaluate_exp(hammer, &build_aff_vec(hammer, String::from(&inst[2..inst.len()]), MAX_STARS+1)?);
                hammer.txt_result.push_str(&format!("cmp rax, 0\nje _end_condition_{}\n", hammer.blocs_index));
            }else{
                end_txt = format!("_real_end_condition_{}:\n", cond_index);
                hammer.cond_index_stack.pop();
            }
        }else if inst == "loop" {
            hammer.txt_result.push_str(&format!("_loop_{}:\n", hammer.blocs_index));
            end_txt = format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index);
            hammer.loop_index_stack.push(hammer.blocs_index);
        }else if inst.starts_with("while") {
            hammer.txt_result.push_str(&format!("_loop_{}:\n", hammer.blocs_index));
            end_txt = format!("jmp _loop_{}\n_end_loop_{}:\n", hammer.blocs_index, hammer.blocs_index);
            hammer.loop_index_stack.push(hammer.blocs_index);
            evaluate_exp(hammer, &build_aff_vec(hammer, String::from(&inst[5..inst.len()]), MAX_STARS+1)?);
            hammer.txt_result.push_str(&format!("cmp rax, 0\nje _end_loop_{}\n", hammer.blocs_index));
        }else if inst != ""{
            return Err(format!("Line {}: Not implemented yet.", get_ln()))
        }
        hammer.jumps_stack.push(Jump::new(hammer.jumps_stack.val().stack_index, end_txt, hammer.blocs_index));
        hammer.blocs_index += 1;
        Ok(())
    }

    fn handle_tab_type_dec(hammer: &mut Hammer, var: &mut Vec<&str>, type_var: &mut Type) -> Result<(), String> {
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
                        hammer.txt_result.push_str(&format!("mov {}[_stack + {} + {}*{}], {}\n", hammer.size[&size].long, previous_data.1, size, j, hammer.stack_index));
                        hammer.stack_index += size*tab_size;
                    }
                    previous_data.0 = tab_size*size;
                },
                Err(_) => return Err(format!("Line {}: You didn't write a correct number between the brackets", get_ln()))
            }
            previous_data.1 = stack_index;
        }
        Ok(())
    }


    fn tab_analyse(hammer: &Hammer, var_s: &mut String) -> Result<Vec::<Vec<(Adress, u8)>>, String>{
        let var: Vec::<&str> = var_s.split("[").collect();
        let mut res = Vec::<Vec<(Adress, u8)>>::new();
        for i in 1..var.len() {
            if var[i] == "" || var[i].chars().last().unwrap() != ']'{
                return Err(format!("Line {}: You forgot to close with '['", get_ln()))
            }
            res.push(build_aff_vec(hammer, String::from(&var[i][0..var[i].len()-1]), 0)?);
        }
        *var_s = String::from(var[0]);
        Ok(res)
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
                    }else{
                        return Err(format!("Line {}: Not implemented yet.", get_ln()));
                    }
                }
            }
        } 
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
        let mut args = Vec::<Vec::<(Adress, u8)>>::new();
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
        });
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
            assert_prof(&hammer.get_var_def_by_name(&var1), nb_stars as u32)?;
            let var2 = split.join("=").replace(" = ", "=");
            let addr = hammer.get_addr(&var1);
            let struct_addr = Adress{val: addr, squares: Some(tab_vec), nb_stars: nb_stars};
            evaluate_exp(hammer,&build_aff_vec(hammer, var2, hammer.get_var_def_by_name(&var1).type_var.stars)?);
            hammer.txt_result.push_str("push rax\n");
            evaluate_exp(hammer, &vec!{(struct_addr, 1)});
            hammer.txt_result.push_str("pop rbx\nmov dword[_stack + rax], ebx\n");
            Ok(())
        }else {
            return Err(format!("Line {}: The variable {} doesn't exists.", get_ln(), var1));
        }
       
    }

    fn assert_prof(var: &VariableDefinition, prof: u32) -> Result<(), String> {    
        if var.type_var.stars < prof {
            return Err(format!("Line {}: The variable {} can be dereference only {} time, you did it {} time", get_ln(), var.name, var.type_var.stars, prof))
        }
        Ok(())
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


    fn build_aff_vec(hammer: &Hammer, mut string_exp: String, nb_stars_await: u32) -> Result<Vec::<(Adress, u8)>, String>{
        if string_exp == String::from("") || string_exp.contains("_") && is_in_a_string(&string_exp, "_".to_string()){
            return Err(format!("Line {}: Syntax error.", get_ln()));
        }
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
        let exp: Vec::<String>  = string_exp.split("_").map(String::from).collect();
        let mut exp_res = Vec::<String>::new();
        let mut current_element = String::new(); 
        let mut cant_be_op = true;
        let mut neg_count: u32 = 0;
        for i in 0..exp.len(){
            let word = exp[i].trim().to_string();
            if word != "" {
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
                    cant_be_op = false;
                    current_element.push_str(&word);
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
        exp_res = hammer.tools.convert_in_postfix_exp(exp_res);
        let mut res = Vec::<(Adress, u8)>::new();
        for elt in exp_res.iter(){
            add_element_in_aff_exp(hammer, elt.to_string(), &mut res, nb_stars_await)?;
        }
        Ok(res)
    }

    fn is_in_a_string(_exp: &String, _part: String) -> bool {
        false
    }

    fn add_element_in_aff_exp(hammer: &Hammer, mut current_element: String, exp: &mut Vec::<(Adress, u8)>, nb_stars_await: u32) -> Result<(), String>{
        if current_element == ""{
            return Err(format!("Line {}: Syntax error.", get_ln()));
        }
        if hammer.tools.is_operator(&current_element){ 
            exp.push((Adress{val: hammer.tools.ascii_val(&current_element), squares: None, nb_stars: 0}, 2))
        }else{
            let mut content = Vec::<i32>::new();
            let mut tab_vec = Vec::<Vec<(Adress, u8)>>::new();
            let interpretation = get_element_interpretation(hammer, &mut current_element, &mut content, &mut tab_vec)?;
            match interpretation {
                Interp::NUMBER => exp.push((Adress::new(content[0]), 0)),
                Interp::VARIABLE => {
                    content[1] += tab_vec.len() as i32;
                    if nb_stars_await != MAX_STARS+1 && (hammer.get_var_def_by_name(&current_element).type_var.stars + (content[1] == -1) as u32 - (((content[1] != -1) as i32)*content[1]) as u32) != nb_stars_await{
                        return Err(format!("Line {}: The two types are incompatibles.", get_ln()));
                    }
                    content[1] -= tab_vec.len() as i32;
                    exp.push((Adress{val: hammer.get_addr(&current_element), squares: Some(tab_vec), nb_stars: content[1]}, 1));
                }
                Interp::CHARACTER => exp.push((Adress::new(content[0]), 0)),
            }
        }
        Ok(())
    }


    fn get_element_interpretation(hammer: &Hammer, element: &mut String, content: &mut Vec::<i32>, tab_vec: &mut Vec::<Vec<(Adress, u8)>>) -> Result<Interp, String>{
        match element.parse::<i32>(){
            Ok(number) => {
                content.push(number);
                return Ok(Interp::NUMBER)
            }
            Err(_e) => {
                let size = 0;
                let nb_stars = get_prof_pointer(element, true)?;
                if hammer.var_exists(&(element.split("[").next().unwrap())){
                    content.push(size);
                    content.push(nb_stars);
                    *tab_vec = tab_analyse(hammer, element)?;
                    return Ok(Interp::VARIABLE);
                }else{
                    match from_char_to_number(&element){
                        Some(val) => {
                            content.push(val as i32);
                            return Ok(Interp::CHARACTER)
                        }
                        _ => return Err(format!("Line {}: we found an incorrect value: {}", get_ln(), element))
                    }
                    
                }
            }
        }
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

    
    fn insert_macro_call_in_txt(hammer: &mut Hammer, macro_call: MacroCall) {
        let mut i = 0;
        for arg in &macro_call.args{
            evaluate_exp(hammer, arg);
            hammer.txt_result.push_str(&format!("mov [_stack + {} + {}], rax\n", hammer.stack_index, i*8));
            i += 1;
        }
        hammer.txt_result.push_str(&format!("{} ", macro_call.macro_name));
        for j in 0..i {
            hammer.txt_result.push_str(&format!("[_stack + {} + {}] ", hammer.stack_index, j*8));
        }
        hammer.txt_result.push_str("\n");
    }

    fn evaluate_exp(hammer: &mut Hammer, exp: &Vec<(Adress, u8)>) {
        for elt in exp{
            if elt.1 == 2{
                hammer.txt_result.push_str(&format!("pop r11\npop r10\nmov r12, {}\ncall _operation\npush rax\n", elt.0.val));
            }else{
                if elt.1 == 1{
                    let mut stars = hammer.addr_list[&elt.0.val].type_var.stars as i32;
                    stars -= elt.0.nb_stars;
                    deref_tab(hammer, &elt.0, &mut stars);
                    if elt.0.nb_stars == -1 {
                        hammer.txt_result.push_str(&format!("push rax\n"));
                    }else if elt.0.nb_stars == 0{
                        if stars == 0{
                            let size_def = hammer.get_size_def(elt.0.val);
                            hammer.txt_result.push_str(&format!("{} rax, {}[_stack + rax]\npush rax\n", size_def.mov, size_def.long));
                        }else{
                            hammer.txt_result.push_str("movsx rax, dword[_stack + rax]\npush rax\n");
                        }
                    }else{
                        hammer.txt_result.push_str(&format!("_deref {}\n", elt.0.nb_stars));
                        if stars == 0{
                            let size_def = hammer.get_size_def(elt.0.val);
                            hammer.txt_result.push_str(&format!("{} rax, {}[_stack + rax]\npush rax\n", size_def.mov, size_def.long));
                        }else{
                            hammer.txt_result.push_str("movzx rax, dword[_stack + rax]\npush rax\n");
                        }
                    }
                }else{
                    hammer.txt_result.push_str(&format!("mov rax, {}\npush rax\n", elt.0.val));
                }       
            }
        }
        hammer.txt_result.push_str("pop rax\n");
        hammer.txt_result = hammer.txt_result.replace("push rax\npop rax\n", "");
    }

    fn deref_tab(hammer: &mut Hammer, var: &Adress, stars: &mut i32) {
        hammer.txt_result.push_str(&format!("mov rbx, {}\n", var.val));
        for vec in var.squares.as_ref().unwrap().iter() {
            evaluate_exp(hammer, vec);
            hammer.txt_result.push_str("mov rcx, 4\nmul rcx\nadd rbx, rax\nmov rax, rbx\n_deref 1\nmov rbx, rax\n");
            *stars -= 1;
        }
        hammer.txt_result.push_str("mov rax, rbx\n");
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
    
    fn break_keyword(hammer: &mut Hammer, _rest_of_line: &String) -> Result<(), String>{
        hammer.txt_result.push_str(&format!("jmp _end_loop_{}\n", hammer.loop_index_stack.val()));
        Ok(())
    }

    fn continue_keyword(hammer: &mut Hammer, _rest_of_line: &String) -> Result<(), String> {
        hammer.txt_result.push_str(&format!("jmp _loop_{}\n", hammer.loop_index_stack.val()));
        Ok(())
    }

}