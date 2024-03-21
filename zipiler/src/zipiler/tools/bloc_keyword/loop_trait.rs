use crate::zipiler::prog_manager::prog_manager::ProgManager;



pub trait LoopTrait {

    fn end_loop(&self, pm: &mut ProgManager) -> String {
        format!("
jmp begin_loop_{id}
end_loop_{id}:", id=pm.bloc_id())
    }

    fn compare_exp(&self, pm: &mut ProgManager) -> String {
        format!("
pop rax
and rax, rax
je end_loop_{}", pm.bloc_id())
    }

    fn new_keyword(&self, kw: &str, pm: &mut ProgManager) -> String {
        match kw {
            "for" => String::new(),     
            "while" | "do" => format!("\nbegin_loop_{}:", pm.bloc_id()),
            _ => panic!("Unknow keyword in a for loop: {kw}")
        }
    }

}