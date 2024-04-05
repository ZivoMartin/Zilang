
use crate::zipiler::tokenizer::include::MemZone;

use super::include::*;

pub struct ProgManager {
    /// Link class name to class definitions
    pub class_name_map: HashMap<String, Class>,
    /// Link var name to a stack of variable address on the stack. The top is the good one
    pub var_name_map: HashMap<String, Stack<usize>>,
    /// Link var address to a stack of variable definition. The top is the good one, two variable can have the
    /// same address if there are not on the same stage 
    pub var_map: HashMap<usize, Stack<VariableDefinition>>,
    /// Link name to a stack of func address on the mem prog
    pub func_name_map: HashMap<String, Stack<usize>>,
    /// Link an address to a function definition
    pub func_map: HashMap<usize, Function>,
    /// The top of the stack memory
    pub stack_index: usize,
    /// The top of the prog memory index
    pub progmem_index: usize,
    /// The top of the heap
    pub heap_index: usize,
    /// The id of the next bloc
    pub bloc_id: u128,
    /// Used for counting the mount of if statment in a if serie
    pub if_count: u32,
    /// Store all the information about the jump through the blocs
    pub jump_stack: Stack<Jump>,
    /// Contains the address of the function we are currently defining
    pub current_func: Option<usize>,
    /// Link a type id (index to access the vec) to type name
    pub titn: Vec::<String>,
    /// Link a type name to his id and his size. For a class the size gonna be the size of a pointer, the
    /// fields of the class are stored on the heap
    pub tnti: HashMap<String, (u8, usize)>,  // Size, id
    /// The current stage
    pub stage: u32,
    /// Name of the class we are currently defining
    pub current_class: String,
    /// The current line of the script.
    line_number: u64
}

impl ProgManager {

    pub fn new() -> ProgManager {
        ProgManager {
            class_name_map: HashMap::new(),
            var_name_map: HashMap::new(),
            var_map: HashMap::new(),
            func_name_map: HashMap::new(),
            func_map: HashMap::new(),
            titn: build_base_type_vec(),
            tnti: build_tab_size_map(),
            stack_index: 0,
            heap_index: 0,
            progmem_index: 0,
            bloc_id: 0,
            if_count: 0,
            jump_stack: Stack::init(Jump::new(0)),
            current_func: None,
            stage: 0,
            current_class: String::new(),
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

    pub fn deref_var(&self, size: usize, stars: i32, spot: MemZone) -> String {
        if stars > 0 {
            match spot {
                MemZone::Heap => format!("mov {}, {}[_heap + rax]", RAX_SIZE[size], ASM_SIZES[size]),
                MemZone::Stack => format!("\n_deref_{} {}", ASM_SIZES[size], stars)
            }
        }else{
            String::new()
        }
    }

    

    pub fn handle_arg(&mut self, f: &Function, stars: i32, nth: usize) -> Result<String, String> {
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

    pub fn end_prog(&self) {
        println!("{} {}", self.si(), self.hi())
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
    Vec::from(TYPE_LIST).iter().map(|e| e.to_string()).collect()
}

fn build_tab_size_map() -> HashMap<String, (u8, usize)> {
    let mut res = HashMap::new();
    res.insert(String::from("int"), (4, 0));
    res.insert(String::from("char"), (1, 1));
    res.insert(String::from("void"), (0, 2));
    res
}


