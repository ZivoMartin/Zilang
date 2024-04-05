use super::include::*;

/// Simply useless asm we remove from each result
static ASM_TO_REMOVE: [&str; 2] = [
    "push rax            ; Then save the result\npop rax",
    "push rax\npop rax"
    ];


pub struct ExpTools {
    /// The operator stack, uses for building the postfix expression.
    op_stack: Stack<String>,
    /// The postfix expression builded during the evaluation of the expression
    pf_exp: Vec<ExpTokenType>,
    /// Associate an operator with his priority, TODO: make is global.
    operator_priority: HashMap<String, u8>,
    /// Associate an operator with his id, TODO: make is global.
    op_id_map: HashMap<String, u8>,
    /// The number of bytes we decall esp during the building of the pf exp
    esp_decal: i64,
    /// The number of stars of the entire expression.
    stars: i32
}

enum ExpTokenType {
    /// An operator with his id
    Operator(u8),
    /// A number with his value.
    Number(i128),
    /// A value, a boolean indicates if its a reference and the size of the value.
    Ident(bool, u8, u8)
}



impl Tool for ExpTools {
    
    fn new_token(&mut self, token: Token, pm: &mut ProgManager) -> Result<String, String>{
        let mut res = String::new();
        match token.token_type {
            TokenType::Number => self.new_number(token.content.parse::<i128>().unwrap()),
            TokenType::RaiseComplexChar(v) => self.new_number(v as i128),
            TokenType::Operator => self.new_operator(token.content),
            TokenType::Symbol => self.new_parenthesis(token.content),
            TokenType::MemorySpot(nb_deref, stars, size, spot) => self.new_cident(nb_deref==-1, stars, size, spot)?,
            TokenType::FuncCall(addr) => res = self.new_funccall(pm, addr)?,
            _ => pm.panic_bad_token("expression", token)
        }
        Ok(res)
    }

    fn new(_pm: &mut ProgManager) -> Box<dyn Tool> {
        Box::from(ExpTools{
            op_stack : Stack::new(),
            pf_exp : Vec::new(),
            operator_priority: build_prio_map(),
            op_id_map: build_op_id(),
            esp_decal: 0,
            stars: 0
        })
    }

    /// The expressions raise the number of stars of the expression. The result is pushed on the stack.
    /// Before doing anything begin by empty the operator stack.
    fn end(&mut self, pm: &mut ProgManager) -> Result<(TokenType, String), String> {
        while self.op_stack.size() != 0 {
            self.push_op_val();
        }
        let asm = self.build_asm(pm);
        Ok((TokenType::RaiseExpression(self.stars), asm))
    }
    
}


impl ExpTools {

    fn new_funccall(&mut self, pm: &ProgManager, addr: usize) -> Result<String, String> {
        let f = pm.get_func_by_addr(addr);
        let stars = f.return_type().stars();
        self.new_cident(true, stars as i32, pm.get_type_size(stars as i32, f.return_type().name()), 1)?;
        Ok(String::from("\npush rax"))
    }

    /// Called when we find a new operator, pop operators on the stack as long as their priority is greater than it.
    /// Then push itself on the op stack.
    fn new_operator(&mut self, content: String) {
        while !self.op_stack.is_empty() && 
              self.op_stack.val().unwrap() != "(" && 
              self.get_priority(self.op_stack.val().unwrap()) >= self.get_priority(&content){
            self.push_op_val();
        }
        self.op_stack.push(content);
    }

    /// Called when we catch a number, take the value of this number in parameter and push it on the pf exp.
    fn new_number(&mut self, number: i128) {
        self.pf_exp.push(ExpTokenType::Number(number));
    }

    /// ( -> Push it on the op stack
    /// ) -> pop the operators til we pop an opening bracket
    pub fn new_parenthesis(&mut self, par: String) {
        match &par as &str {
            "(" => self.op_stack.push(par),
            ")" => {
                while self.op_stack.val().unwrap() != "(" {
                    self.push_op_val();
                }
                self.op_stack.pop();
            }
            _ => panic!("Unknow parenthesis: {par}")
        } 
    }

    /// Called when we detect a comple ident, take in parameter the is_ref boolean,, The depth of the type of the 
    /// ident (his number of stars) and the size of the ident. Gonna verifie if the number 
    /// of stars is the same of the number of stars of the entire expression, or if this is the first time we 
    /// find a value with a depth superior than 0. Then gonna push the value with the size and the is_ref boolean. 
    fn new_cident(&mut self, is_ref: bool, stars: i32, size: u8, spot: u8) -> Result<(), String> {
        if self.stars == 0 {
            self.stars = stars;
        }else if stars != 0 && stars != self.stars {
            return Err(String::from("Bad type."))
        }
        self.pf_exp.push(ExpTokenType::Ident(is_ref, size, spot));
        self.esp_decal += 8;
        Ok(())
    }

    /// Return the priority of an operator.
    fn get_priority(&self, op: &String) -> u8 {
        *self.operator_priority.get(op).unwrap_or_else(
            || panic!("This operator doesn't have priority yet: {op}")
        )
    }

    /// Pop the top of the stack and push the op code on the pf expression
    fn push_op_val(&mut self) {
        let val: u8 = *self.op_id_map.get(&self.op_stack.pop().unwrap()).unwrap();
        self.pf_exp.push(ExpTokenType::Operator(val));    
    }

    /// Travel the entire postfix expression, and build the asm for each element. Then returns the entire asm.
    /// Has to save the base value of rsp in rbp cause for pick the element on the stack we have to stay with the same
    /// base value but the stack endure a lot of operation, so we have to save it.
    /// Then gonna pop the result, replace the stack and push the result. Do a little asm oprimisation at the end
    fn build_asm(&self, pm: &ProgManager) -> String {
        let mut nb_ident = 1;
        let mut res = String::new();
        res.push_str("\nmov rbp, rsp");
        for t in self.pf_exp.iter() {
            match t {
                ExpTokenType::Operator(op_code) => self.op_found(&mut res, *op_code),
                ExpTokenType::Number(v) => self.number_found(&mut res, *v),
                ExpTokenType::Ident(is_ref, size, spot) =>  self.cident_found(&mut res, pm, &mut nb_ident, *is_ref, *size, *spot),
            }
        }
        nb_ident -=1 ;
        if nb_ident != 0 {
            res.push_str(&format!("
pop rax
add rsp, {}
push rax", nb_ident*8))
        }
        for a in ASM_TO_REMOVE.iter() {
            res = res.replace(a, "");
        }
        res
    }

    /// Pop the two last value and do the operation
    fn op_found(&self, res: &mut String, op_code: u8) {
        res.push_str(&format!("
mov r12, {op_code}        ; We found an operator, lets do the operation
pop r11             ; First operande
pop r10             ; Second operande
call _operation     ; call the operation
push rax            ; Then save the result"             
        ))       
    }

    /// Simply push the number on the value stack
    fn number_found(&self, res: &mut String, v: i128) {
        res.push_str(&format!("
push {v}            ; We found a number, lets push it"
        ))
    }

    /// Keep the ident on the stack, then if isn't a ref dereference it and push his value on the stack.
    /// Uses esp_decall to know where to find the address.
    fn cident_found(&self, res: &mut String, pm: &ProgManager, nb_ident: &mut i64, is_ref: bool, size: u8, spot: u8) {
        res.push_str(&format!("      
mov rax, [rbp+{}]   ; We found an ident, its on the satck, lets keep it.
{}
push rax", self.esp_decal-*nb_ident*8, if is_ref {String::new()}else{pm.deref_var(size as usize, 1, spot)}
        ));
        *nb_ident += 1;
        
    }

}

fn build_prio_map() -> HashMap<String, u8>{
    let mut res = HashMap::<String, u8>::new();
    for op in vec!["%", "*", "/"].iter() {
        res.insert(String::from(*op), 4);
    }
    for op in vec!("<", "<=", ">", ">=", "==", "!=", "||", "&&").iter() {
        res.insert(String::from(*op), 2);
    }
    res.insert(String::from("+"), 3);
    res.insert(String::from("-"), 3);
    res.insert(String::from(")"), 4);
    res.insert(String::from("("), 5);
    res
}

fn build_op_id() -> HashMap<String, u8> {
    let mut res = HashMap::new();
    for (i, op) in OPERATORS.iter().enumerate() {
        res.insert(op.to_string(), i as u8);
    }
    res
}

