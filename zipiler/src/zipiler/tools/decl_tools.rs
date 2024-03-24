use super::include::*;

pub struct DeclTools {
    addr: usize,
    type_name: String,
    stars: u32,
    aff: bool,
    arr_size: Vec<usize>,
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
            res.push_str("
pop rax"
           );
           res.push_str(&pm.affect_to(self.addr));
        }
        res
    }

    fn new_number(&mut self, n: usize) {
        self.arr_size.push(n);
    }

    fn alloc(&mut self, pm: &mut ProgManager) -> String {
        let mut res = String::new();
        if !self.arr_size.is_empty() {
            let mut previous_data: (usize, usize) = (1, pm.si()-4);
            for i in 0..self.arr_size.len() {
                let save_si = pm.si();
                let tab_size = self.arr_size[i];
                let size = if i == self.arr_size.len()-1 {pm.get_type_size(0, &self.type_name) as usize}else{POINTER_SIZE};
                for j in 0..previous_data.0{
                    res.push_str(&pm.affect_to_wsize(previous_data.1+size*j, size, pm.si()));
                    pm.inc_si(size*tab_size);
                }
                previous_data.0 *= tab_size;
                previous_data.1 = save_si;
            }
            self.stars += self.arr_size.len() as u32;
        }
        res
    }

}
