
pub struct TypeGestion{
    authorized_type: Vec<&'static str>,
    operator_list: Vec<&'static str>
}

impl TypeGestion{

    pub fn new() -> TypeGestion{
        TypeGestion{
            authorized_type: vec!{"BIT", "CHAR", "DATETIME", "DECIMAL", "FLOAT",
            "INT", "MONEY", "NUMERIC", "REAL", "SMALLDATETIME", "SMALLINT", "SMALLMONEY", "TINYINT", "VARCHAR", "BOOL"},
            operator_list: vec!{"<", ">", "<=", ">=", "==", "!=", "(", "AND", "OR"}
        }
    }

    pub fn get_nth_operator(&self, n: i32)->&str{
        self.operator_list[n as usize]
    }

    pub fn operator_exist(&self, op: &str)->bool{
        self.operator_list.contains(&op)
    }

    pub fn is_int(&self, string : &str) -> bool{
        let numbers = "1234567890";
        for chara in string.chars(){
            if !numbers.contains(chara.clone()){
                return false;
            } 
        }
        true
    }

    pub fn is_float(&self, string : &str) -> bool{
        let numbers = "1234567890";
        let mut point = false;
        let mut i = 0;
        for chara in string.chars(){
            if !numbers.contains(chara.clone()){
                if chara == '.' && !point{
                    if i == string.len() - 1{
                        return false;
                    }
                    point = true;
                }else{
                    return false;
                }
            }
            i += 1;
        }
        true
    }

    pub fn good_type_and_good_value(&self, type_value: &str, value: &str) -> bool{
        match type_value{
            "BOOL" => value == "false" || value == "true",
            "STRING" => true,
            "INT" => self.is_int(value),
            _ => self.is_float(value) || type_value.starts_with("VARCHAR")
        }
    }
    

    pub fn is_correct_type(&self, tested_type: &str) -> bool{
        if !tested_type.starts_with("VARCHAR"){
            self.authorized_type.contains(&tested_type) 
        }else{
            let mut t = tested_type.to_string(); 
            for _ in 0..7{
                t.remove(0);
            }
            if t.remove(0) == '(' && t.pop() == Some(')'){
                return self.is_int(&t);
            }else{
                return false;
            }
        }
    }
    
    pub fn descript_a_string_bool(&self, exp: &str) -> bool{
        self.evaluate_postfix_exp(&self.valid_infix_to_postfix(exp))
    }

    fn valid_infix_to_postfix(&self, exp: &str) -> String{
        let mut result = String::new();
        let mut stack = Vec::<&str>::new();
        let split_exp: Vec<&str> = exp.split_whitespace().collect();
        for elt in split_exp{
            if self.operator_list.contains(&elt){
                while stack.len()>0 && stack.last().unwrap().to_string() != String::from("(") && self.get_priority(elt) < self.get_priority(stack.last().unwrap()){
                    result.push_str(&format!(" {}", stack.pop().unwrap()));
                }
                stack.push(elt);
            }else if elt == ")"{
                while stack.last().unwrap().to_string() != String::from("(") {
                    result.push_str(&format!(" {}", stack.pop().unwrap()));
                }
                stack.pop();
            }else{
                result.push_str(&format!(" {}", elt));
            }
        }
        while stack.len() != 0 {
            result.push_str(&format!(" {}", stack.pop().unwrap()));
        }
        result.remove(0);
        result
    }


    fn get_priority(&self, operator: &str) -> i32{
        if operator == "AND" || operator == "OR"{
            return 1;
        }else if operator == "("{
            return 3;
        }else{
            return 2;
        }
    }

    fn evaluate_postfix_exp(&self, exp: &str) -> bool{
        let mut stack = Vec::<&str>::new();
        let split_exp: Vec<&str> = exp.split_whitespace().collect();
        for elt in split_exp{
            if self.operator_list.contains(&elt){
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                if self.compare_to_valid_element(left, elt, right){
                    stack.push("1");
                }else{
                    stack.push("0");
                }
            }else{
                stack.push(elt);
            }
        }
        return stack[0] == "1";
    }

    fn compare_to_valid_element(&self, left_s: &str, operator: &str, right_s: &str) -> bool{
        let left: f32 = String::from(left_s).parse().unwrap_or_default();
        let right: f32 = String::from(right_s).parse().unwrap_or_default();
        match operator{
            "==" => left == right,
            "!=" => left != right,
            ">" => left > right,
            "<" => left < right,
            ">=" => left >= right,
            "<=" => left <= right,
            "AND" => left == 1.0 && right == 1.0,
            "OR" => left == 1.0 || right == 1.0,
            _ => false
        }
    }

    pub fn hash_string_to_number(&self, string: String)->i32{
        let mut result: i32 = 0;
        for chara in string.chars(){
            result += chara as i32;
        }
        result
    }


    pub fn convert_bool_to_number(&self, string: &str) -> String{
        match string{
            "true" => return String::from("1"),
            _ => return String::from("0")
        }
    }
}