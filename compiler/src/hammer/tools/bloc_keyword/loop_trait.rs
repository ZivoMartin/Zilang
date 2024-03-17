use memory::Memory;

use crate::hammer::memory;



pub trait LoopTrait {

    fn end_loop(&self, memory: &mut Memory) -> String {
        format!("
jmp begin_loop_{id}
end_loop_{id}:", id=memory.bloc_id)
    }

    fn compare_exp(&self, memory: &mut Memory) -> String {
        format!("
pop rax
and rax, rax
je end_loop_{}", memory.bloc_id)
    }

    fn new_keyword(&self, kw: &str, memory: &mut Memory) -> String {
        match kw {
            "for" => String::new(),     
            "while" | "do" => format!("\nbegin_loop_{}:", memory.bloc_id),
            _ => panic!("Unknow keyword in a for loop: {kw}")
        }
    }

}