use crate::zipiler::prog_manager::prog_manager::ProgManager;



pub trait LoopTrait {

    fn bi(&self) -> u128;

    fn init(&self, pm: &mut ProgManager) {
        pm.inc_bi();
    }

    fn end_loop(&self, _pm: &mut ProgManager) -> String {
        format!("
jmp begin_loop_{id}
end_loop_{id}:", id=self.bi())
    }

    fn compare_exp(&self, _pm: &mut ProgManager) -> String {
        format!("
pop rax
and rax, rax
je end_loop_{}", self.bi())
    }

    fn new_keyword(&self, kw: &str, _pm: &mut ProgManager) -> String {
        match kw {
            "for" => String::new(),     
            "while" | "do" =>   format!("\nbegin_loop_{}:", self.bi()),
            _ => panic!("Unknow keyword in a for loop: {kw}")
        }
    }

}