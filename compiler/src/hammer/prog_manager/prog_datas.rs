use super::include::*;

pub struct ProgData {
    stack_index: usize,
    bloc_id: u128,
    if_count: u32,
    jump_stack: Stack<Jump>,
    current_file: usize
}


impl ProgData {

    pub fn new() -> ProgData {
        ProgData{
            stack_index: 0,
            bloc_id: 0,
            if_count: 0,
            jump_stack: Stack::init(Jump::new(0)),
            current_file: SCRIPTF
        }
    }

    pub fn bloc_id(&self) -> u128 {
        self.bloc_id
    }

    pub fn cf(&self) -> usize {
        self.current_file
    }

    pub fn if_count(&self) -> u32 {
        self.if_count
    }

    pub fn set_if_count(&mut self, v: u32) {
        self.if_count = v;
    }

    pub fn inc_bi(&mut self) {
        self.bloc_id += 1;
    }

    pub fn inc_if_count(&mut self) {
        self.if_count += 1;
    }

    pub fn si(&self) -> usize {
        self.stack_index
    }

    pub fn in_func(&mut self) {
        self.current_file = FUNCTIONSF;
    }

    pub fn out_func(&mut self) {
        self.current_file = SCRIPTF;
    }

    pub fn jump_in(&mut self) {
        self.jump_stack.push(Jump::new(self.si()));
    }

    pub fn jump_out(&mut self) {
        let last_jump = self.jump_stack.pop().expect("Can t jump out, stack empty");
        for addr in last_jump.addr_to_remove.iter() {
            let var_def = self.var_map.remove(addr).expect("Adress unvalid");
            self.var_name_map
                .get_mut(&var_def.name).expect("The name doesn't exists")
                .pop().expect("The varname stack is empty");
        }
        self.si() = last_jump.stack_index;
    }
}