use super::include::*;

pub struct CIdentTools {
    deref_time: i32,
    name: String,
    for_bracket: bool // If we catch an exp, its determine if its between a bracket or a tupple
}

impl Tool for CIdentTools {

    fn new_token(&mut self, token: Token, _memory: &mut Memory) -> Result<(), String>{
        Ok(match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content)?,
            TokenType::Ident => self.def_ident(token.content),
            TokenType::Brackets => self.open_brackets(),
            TokenType::ExpressionTuple => self.open_tupple(),
            TokenType::Expression => self.new_expression(),
            _ => panic_bad_token("complex ident", token)
        })
    }
    
    fn new() -> Box<dyn Tool> {
        Box::from(CIdentTools {
            deref_time: 0,
            name: String::new(),
            for_bracket: true
        })
    }

   
    fn end(&mut self, memory: &mut Memory) -> Result<(Token, String), String> {
        let var_def = match memory.get_var_def_by_name(&self.name) {
            Ok(var_def) => var_def,
            Err(_) => return Err(format!("{} isn't an axisting variable.", &self.name))
        };
        let stars = var_def.type_var.stars as i32 - self.deref_time;
        if stars < -1 {
            return Err(format!("Bad dereferencment")) // If you want to modifie this line, care it could be dangerous because of the unsafe
        }
        
        let asm = self.build_asm(stars, self.deref_time, &memory, var_def);
        Ok((Token::new(TokenType::ComplexIdent, format!("{stars} {}", var_def.get_size())), asm))
    }
}


impl CIdentTools {


    pub fn new_symbol(&mut self, s: String) -> Result<(), String>{
        if s == "*" {
            self.deref_time += 1;
        }else if s == "&" {
            self.deref_time -= 1;
            if self.deref_time < -1 {
                return Err(String::from("You can't dereferance like this.."))
            }
        }else{
            panic!("Bad symbol for a complxident: {s}")
        }
        Ok(())
    }

    pub fn def_ident(&mut self, name: String){
        self.name = name;
    }

    pub fn open_brackets(&mut self) {
        self.for_bracket = true;
    }

    pub fn open_tupple(&mut self) {
        self.for_bracket = false;
    }

    pub fn new_expression(&mut self) {
        if self.for_bracket {
            println!("New expression for bracket");
            // Todo: Push the result of the exp on the stack ? No, think more
        }else{
            println!("New expression for tupple");
            // Todo: handle func call 
        }
    }

    fn build_asm(&self, _stars: i32, deref_time: i32, memory: &Memory, var_def: &VariableDefinition) -> String {
        let mut res = format!("\nmov rax, {}", var_def.addr);
        res.push_str(&memory.deref_var(var_def.type_var.size as usize, deref_time));
        res.push_str("\npush rax    ; We push the value of a new identificator");
        println!("RES: {res}");
        res
    }

}