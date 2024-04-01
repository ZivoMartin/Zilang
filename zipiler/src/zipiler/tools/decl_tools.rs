use super::include::*;

pub struct DeclTools {
    /// The name of the new variable
    name: String,
    /// The name of the type value of the declaration
    type_name: String,
    /// The number of stars of the type of the declaration
    stars: u32,
    /// Indicate if we are doing instently an affectation or not.
    aff: bool,
    /// If its an array declaration indicate for each stage the number of square
    arr_size: Vec<usize>,
    /// The stack index before the allocation of the entire asked memory
    save_si: usize,
    /// The number of expression on the right (generaly uses for the direct array affectation)
    nb_exp: usize,
    /// If its a direct string affectation, gonna be the aked string
    string: String,
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
            _ => pm.panic_bad_token("declaration", token)
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

    // Raise the address of the new var and returns the asm who affect the right expression on the new variable
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        let addr = pm.si();
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseDeclaration(addr), asm))
    }

}

impl DeclTools {

    /// Called when we raise the data of a complex type. So we receiv an id and we transform it with a name,
    /// and we set the number of stars.
    fn def_type(&mut self, pm: &ProgManager, id: usize, stars: u32) {
        self.type_name = pm.get_type_name_with_id(id);
        self.stars = stars;
    }

    /// Check if the number of stars of the right expression has the same value as the number of stars of our type.
    fn check_exp(&mut self, stars: i32) -> Result<(), String>{
        return if stars as u32 != self.stars {
            Err(String::from("Not the good type"))
        }else{
            self.nb_exp += 1;
            Ok(())
        }
    }

    /// Set the name of the variable
    pub fn def_name(&mut self, name: String) {
        self.name = name;
    }

    /// Called when we catch the equal operator, juste inform the tool that we are doing an affectation
    pub fn def_equal_operator(&mut self) {
        self.aff = true;
    }

    /// If the string is empty, we just pop on the stack all the values of each expression and affect it to our new var.
    /// Otherwise we memorize the built string and affect the address of it to our variable
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

    /// Here the user define a new stage for an array with the size n. We just memorize this information.
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
    
    /// Here we allocate the good place depends of the array of size for each stage. Basically we just create a big
    /// empty zone and actualise the stack index.
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

    /// We juste memorize each byte in the string of the tool. 
    fn handle_string_aff(&mut self, res: &mut String) {
        for b in self.string.as_bytes() {
            res.push_str(&format!("
mov byte[_stack + {}], {b}", self.save_si));
            self.save_si += 1;
        }
    }

    /// If we are building a string, add the symbol to the string.
    fn new_symbol(&mut self, symb: u8) {
        self.string.push(symb as char);
    }
}
