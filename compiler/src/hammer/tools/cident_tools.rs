use crate::hammer::memory::Memory;
use crate::hammer::include::POINTER_SIZE;
use super::program::{Tool, panic_bad_token};
use crate::hammer::tokenizer::include::{TokenType, Token};

pub struct CIdentTools {
    deref_time: i32,
    name: String,
    for_bracket: bool // If we catch an exp, its determine if its between a bracket or a tupple
}

impl Tool for CIdentTools {

    fn new_token(&mut self, token: Token, memory: &mut Memory) -> Result<Option<Token>, String>{
        match token.token_type {
            TokenType::Symbol => self.new_symbol(token.content),
            TokenType::Ident => self.def_ident(token.content),
            TokenType::Brackets => self.open_brackets(),
            TokenType::ExpressionTuple => self.open_tupple(),
            TokenType::Expression => self.new_expression(),
            TokenType::EndToken => return Ok(Some(Token::new(TokenType::ComplexIdent, self.end(memory)?))),
            _ => panic_bad_token("complex ident", token)
        }
        Ok(None)
    }

    fn new() -> Box<dyn Tool> {
        Box::from(CIdentTools {
            deref_time: 0,
            name: String::new(),
            for_bracket: true
        })
    }
}


impl CIdentTools {


    pub fn new_symbol(&mut self, s: String) {
        if s == "*" {
            self.deref_time += 1;
        }else if s == "&" {
            self.deref_time -= 1;
        }else{
            panic!("Bad symbol for a complxident: {s}")
        }
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

    // Fonctionnement: Le problème est comment expliquer à l'appelant quel est la valeur de l'ident qui vient d'etre identifié ?
    // On calcule sa valeur en assembleur avant de compute l'appelant, exemple dans 2*a avant de compute 2*a on compute a
    // puis on push a sur la stack on se retrouve donc avec 2 a * a la fin de l'expression mais on a push a sur la stack, plus
    // qu'a recuperer la valeur de a sur la stack lors de la traduction en assembleur. Il est egalement necessaire de savoir
    // ou trouver les valeurs par rapport à esp, pour cela on va compter dans l'expression de combien nous avons avancé esp,
    // puis a chaque identificateur indiquer en partant du bas de la pile l'adresse des valeurs. Exemple pour 2*a+b
    // avec a et b des ints, on a esp decalé de 8, et l'expression devient alors 2*|0+|4.
    pub fn end(&mut self, memory: &Memory) -> Result<String, String> {
        let var_def = match memory.get_var_def(&self.name) {
            Ok(var_def) => var_def,
            Err(_) => return Err(format!("{} isn't an axisting variable.", &self.name))
        };
        let stars = var_def.type_var.stars as i32 - self.deref_time;
        
        // Todo: Push en assembleur l'extraction de la valeur de var_def et la push dans la stack.
        Ok(format!("{}",
        if stars == 0 {
            var_def.type_var.size
        }else if stars < -1 {
            return Err(format!("Bad dereferencment")) // If you want to modifie this line, care it could be dangerous because of the unsafe
        }else{
            POINTER_SIZE
        }))
    }

}