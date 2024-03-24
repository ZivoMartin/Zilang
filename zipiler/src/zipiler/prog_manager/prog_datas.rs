use super::{include::*, prog_manager::ProgManager};



impl ProgManager {


    pub fn bloc_id(&self) -> u128 {
        self.bloc_id
    }

    pub fn inc_si(&mut self, n: usize) {
        self.stack_index += n;
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
        self.stage += 1;
        self.stack_index = 0;
    }

    pub fn out_func(&mut self) {
        self.stage -= 1;
    }

    pub fn jump_in(&mut self) {
        self.jump_stack.push(Jump::new(self.si()));
    }

    pub fn jump_out(&mut self) {
        let last_jump = self.jump_stack.pop().expect("Can t jump out, stack empty");
        for addr in last_jump.addr_to_remove.iter() {
            let addr_stack = self.var_map.get_mut(addr).expect("Adress unvalid"); 
            let var_def = addr_stack.pop().expect("satck empty");
            if addr_stack.is_empty() {
                self.var_map.remove(addr);
            };
            self.var_name_map
                .get_mut(var_def.name()).expect("The name doesn't exists")
                .pop().expect("The varname stack is empty");
        }
        self.stack_index = last_jump.stack_index;
    }

}