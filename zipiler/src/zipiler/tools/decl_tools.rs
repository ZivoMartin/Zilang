use super::include::*;

pub struct DeclTools {
    addr: usize,
    type_name: String,
    stars: u32,
    aff: bool,
    arr_size: Vec<usize>,
    save_si: usize,
    nb_exp: usize
}

impl Tool for DeclTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::RaiseComplexType(id, stars, _) => self.def_type(pm, id, stars as u32),
            TokenType::Ident => self.def_name(token.content, pm),
            TokenType::Operator => self.def_equal_operator(),
            TokenType::RaiseExpression(stars) => self.check_exp(stars)?,
            TokenType::Number => self.new_number(token.content.parse::<usize>().unwrap()),
            _ => panic_bad_token("declaration", token)
        }
        Ok(String::new())
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(DeclTools {
            addr: 0,
            type_name: String::new(),
            stars: 0,
            save_si: 0,
            aff: false,
            arr_size: Vec::new(),
            nb_exp: 0
        })
    }

    // Raise the address of the new var
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseDeclaration(self.addr), asm))
    }

}

impl DeclTools {



    pub fn def_type(&mut self, pm: &ProgManager, id: usize, stars: u32) {
        self.type_name = pm.get_type_name_with_id(id);
        self.stars = stars;
    }

    pub fn check_exp(&mut self, stars: i32) -> Result<(), String>{
        return if stars as u32 != self.stars {
            Err(String::from("Not the good type"))
        }else{
            self.nb_exp += 1;
            Ok(())
        }
    }

    pub fn def_name(&mut self, name: String, pm: &mut ProgManager) {
        self.addr = pm.new_var(self.type_name.clone(), name, self.stars);
    }

    pub fn def_equal_operator(&mut self) {
        self.aff = true;
    }

    fn build_asm(&mut self, pm: &mut ProgManager) -> String {
        let mut res = self.alloc(pm);
        if self.aff {
           for i in 0..self.nb_exp {
            res.push_str(&format!("
mov rax, [rsp + {}]
mov dword[_stack + r15 + {}], eax", (self.nb_exp-i-1)*8, self.save_si));
            self.save_si += 4;
           }
        }
        res.push_str(&format!("
add rsp, {}", self.nb_exp*8));
        res
    }

    fn new_number(&mut self, n: usize) {
        self.arr_size.push(n);
    }

    fn alloc(&mut self, pm: &mut ProgManager) -> String {
        let mut res = String::new();
        self.save_si = pm.si()- ((pm.si()!=0) as usize) * 4;
        let mut stack_val = 1;
        for (i, n) in self.arr_size.iter().enumerate() {
            let save_si = pm.si();
            let size = if i == self.arr_size.len()-1 {pm.get_type_size(0, &self.type_name) as usize}else{POINTER_SIZE};
            for j in 0..stack_val{
                res.push_str(&pm.affect_to_wsize(self.save_si+size*j, size, pm.si()));
                pm.inc_si(size*n);
            }
            self.save_si = save_si;
            stack_val *= n;
        }
        self.stars += self.arr_size.len() as u32;
        res
    }

}
