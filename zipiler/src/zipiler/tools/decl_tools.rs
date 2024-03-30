use super::include::*;

pub struct DeclTools {
    type_name: String,
    stars: u32,
    aff: bool,
    arr_size: Vec<usize>,
    save_si: usize,
    nb_exp: usize,
    string: String,
    name: String
}

impl Tool for DeclTools {

    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        match token.token_type {
            TokenType::RaiseComplexType(id, stars, _) => self.def_type(pm, id, stars as u32),
            TokenType::Ident => self.def_name(token.content),
            TokenType::Operator => self.def_equal_operator(),
            TokenType::RaiseExpression(stars) => self.check_exp(stars)?,
            TokenType::RaiseComplexChar(char) => self.new_symbol(char),
            TokenType::Number => self.new_number(token.content.parse::<usize>().unwrap()),
            _ => panic_bad_token("declaration", token)
        }
        Ok(String::new())
    }


    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(DeclTools {
            type_name: String::new(),
            stars: 0,
            save_si: 0,
            aff: false,
            arr_size: Vec::new(),
            nb_exp: 0,
            string: String::new(),
            name: String::new()
        })
    }

    // Raise the address of the new var
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let addr = pm.si();
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseDeclaration(addr), asm))
    }

}

impl DeclTools {

    fn new_symbol(&mut self, symb: u8) {
        self.string.push(symb as char);
    }

    fn def_type(&mut self, pm: &ProgManager, id: usize, stars: u32) {
        self.type_name = pm.get_type_name_with_id(id);
        self.stars = stars;
    }

    fn check_exp(&mut self, stars: i32) -> Result<(), String>{
        return if stars as u32 != self.stars {
            Err(String::from("Not the good type"))
        }else{
            self.nb_exp += 1;
            Ok(())
        }
    }

    pub fn def_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn def_equal_operator(&mut self) {
        self.aff = true;
    }

    fn build_asm(&mut self, pm: &mut ProgManager) -> String {
        let mut res = self.alloc(pm);
        if self.aff {
            if self.string.is_empty() {
                self.memorize_all_exp(&mut res)
            }else{
                self.handle_string_aff(&mut res)
            }
        }
        
        res
    }

    fn new_number(&mut self, n: usize) {
        self.arr_size.push(n);
    }

    fn memorize_all_exp(&mut self, res: &mut String) {
        for i in 0..self.nb_exp {
            res.push_str(&format!("
mov rax, [rsp + {}]
mov dword[_stack + r15 + {}], eax", (self.nb_exp-i-1)*8, self.save_si));
            self.save_si += 4;
        }
        res.push_str(&format!("
        add rsp, {}", self.nb_exp*8));
    }
    
    fn alloc(&mut self, pm: &mut ProgManager) -> String {

        self.save_si = pm.si();
        self.stars += self.arr_size.len() as u32;
        pm.new_var(self.type_name.clone(), self.name.clone(), self.stars);
        let mut res = String::new();
        let mut stack_val = 1;
        for (i, n) in self.arr_size.iter().enumerate() {
            let save_si = pm.si();
            let size = if i == self.arr_size.len()-1 {pm.get_type_size(0, &self.type_name) as usize}else{POINTER_SIZE};
            for j in 0..stack_val{
                res.push_str(&pm.affect_to_wsize(self.save_si+size*j, POINTER_SIZE, pm.si()));
                pm.inc_si(size*n);
            }
            self.save_si = save_si;
            stack_val *= n;
        }
        res
    }

    fn handle_string_aff(&mut self, res: &mut String) {
        for b in self.string.as_bytes() {
            res.push_str(&format!("
mov byte[_stack + {}], {b}", self.save_si));
            self.save_si += 1;
        }
    }

}
