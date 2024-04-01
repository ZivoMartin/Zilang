
use super::include::*;

pub struct ProgManager {
    pub var_name_map: HashMap<String, Stack<usize>>,
    pub var_map: HashMap<usize, Stack<VariableDefinition>>,
    pub func_name_map: HashMap<String, Stack<usize>>,
    pub func_map: HashMap<usize, Function>,
    pub stack_index: usize,
    pub bloc_id: u128,
    pub if_count: u32,
    pub jump_stack: Stack<Jump>,
    pub current_func: Option<usize>,
    pub titn: Vec::<String>,
    pub tnti: HashMap<String, (u8, usize)>,
    pub preload: String,
    pub stage: u32,
    line_number: u64
}

impl ProgManager {

    pub fn new() -> ProgManager {
        ProgManager {
            var_name_map: HashMap::new(),
            var_map: HashMap::new(),
            func_name_map: HashMap::new(),
            func_map: HashMap::new(),
            titn: build_base_type_vec(),
            tnti: build_tab_size_map(),
            stack_index: 0,
            bloc_id: 0,
            if_count: 0,
            jump_stack: Stack::init(Jump::new(0)),
            preload: String::from("\npreload:"),
            current_func: None,
            stage: 0,
            line_number: 1
        }
    }

    

    pub fn _affect_to(&self, addr: usize) -> String {
        let size = self.get_var_def(&addr).unwrap().type_var().size() as usize;
        format!("\nmov {}[_stack + {STACK_REG} + {}], {}", ASM_SIZES[size], addr, RAX_SIZE[size])
    }

    pub fn affect_to_wsize(&self, addr: usize, size: usize, val: usize) -> String {
        format!("\nmov {}[_stack + {STACK_REG} + {}], {}", ASM_SIZES[size], addr, val)        
    }

    pub fn deref_var(&self, size: usize, stars: i32) -> String {
        if stars > 0 {
            format!("\n_deref_{} {}", ASM_SIZES[size], stars)
        }else{
            String::new()
        }
    }

    

    pub fn handle_arg(&mut self, f_name: &str, stars: i32, nth: usize) -> Result<String, String> {
        let f = self.get_func_by_name(f_name)?;
        if f.args()[nth as usize].stars() as i32 != stars {
            return Err("Not the good type for the call.".to_string())
        }
        let size = self.get_type_size(stars, &f.args()[nth as usize].name()) as usize;
        let res = format!("
pop rax
mov {}[_stack + {STACK_REG} + {}], {}", ASM_SIZES[size], self.si(), RAX_SIZE[size]);
        self.stack_index += size;
        Ok(res)
    }

    pub fn good_nb_arg(&mut self, name: &str, nb_arg: u8) -> Result<(), String>{
        if self.get_func_by_name(name)?.nb_arg() != nb_arg as usize {
            Err(String::from("not the good number of arg"))
        }else{
            Ok(())
        }
    }

    pub fn preload(&mut self, script: String) {
        self.preload.push_str(&script)
    }

    pub fn _get_preload(&self) -> &String {
        &self.preload
    }

    pub fn end_prog(&mut self) {
        self.preload.push_str("\nret");
    }

    pub fn is_in_func(&self) -> bool {
        self.current_func.is_some()
    }

    pub fn new_line(&mut self) {
        self.line_number += 1;
    }

    pub fn line_number(&self) -> u64 {
        return self.line_number;
    }

    pub fn panic_bad_token(&self, receiver: &str, token: Token) {
        panic!("Line {}: Unknow token type for a {receiver}: {:?}    {}", self.line_number(), token.token_type, token.content)
    }

}

fn build_base_type_vec() -> Vec<String> {
    vec!("int", "char", "void").iter().map(|e| e.to_string()).collect()
}

fn build_tab_size_map() -> HashMap<String, (u8, usize)> {
    let mut res = HashMap::new();
    res.insert(String::from("int"), (4, 0));
    res.insert(String::from("char"), (1, 1));
    res.insert(String::from("void"), (0, 2));
    res
}


