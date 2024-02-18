use crate::system::System;
use std::collections::HashMap;
use crate::type_gestion::TypeGestion;

pub struct Interpreteur {
    system: System,
    authorized_char_for_variable: &'static str,
    type_gestion: TypeGestion
}

impl Interpreteur {
    pub fn new() -> Interpreteur{
        Interpreteur{
            system: System::new(),
            authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
            type_gestion: TypeGestion::new()
        }
    }

    pub fn sqlrequest(&mut self, mut req: String, _json: bool) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if req != "" {
            req = req.replace(",", " , ");
            while req.contains("  ") {
                req = req.replace("  ", " ");
            }
            let mut vect_req: Vec<&str> = req.split(" ").map(str::trim).collect();
            let type_request = vect_req.remove(0);
            match type_request{
                "DROP" => self.drop_req(vect_req)?,
                "CREATE" => self.create_req(vect_req)?,
                "INSERT" => {
                    if vect_req.len() >= 5 && vect_req.remove(0) == "INTO" && vect_req.contains(&"VALUES"){
                        self.insert_request(vect_req)?
                    }else{
                        return Err(String::from("Invalid request."));
                    }
                }
                "DELETE" => self.delete_line(vect_req)?,
                "SELECT" => return self.select_request(vect_req),
                _ => return Err(format!("{} is unnknow by the system.", type_request))
            }
        }
        Ok(None)
    }
    



    fn drop_req(&mut self, vect_req: Vec::<&str>) -> Result<(), String>{
        if vect_req.len() >= 2{
            let mut arguments = HashMap::<&str, &str>::new();
            arguments.insert(":request", "DELETE_TABLE");
            match vect_req[0]{
                "TABLE" => {
                    for table_to_drop in vect_req.iter().skip(1){
                        arguments.insert(":table_name", table_to_drop); // We insert the table to drop
                        self.system.new_request(arguments.clone())?;    // We drop it
                    }
                }
                _ => {}
            }
            return Ok(())
        }
        Err(format!("DROP {} isn't a valid command.", vect_req.join(" ")))
    }



    fn select_request(&mut self, mut vect_req: Vec<&str>) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 3 {
            let mut arguments = HashMap::<String, String>::new();
            let mut result = HashMap::<&str, &str>::new();
            arguments.insert(String::from(":request"), String::from("SELECT"));
            arguments.insert(String::from(":asked"), String::from("*"));
            match vect_req[0]{
                "*" => {
                    vect_req.remove(0);
                }
                _ => {
                    let mut asked = String::new();  
                    loop {
                        asked = asked + &vect_req.remove(0).to_string() + "/";
                        if vect_req[0] != "," {
                            break;
                        }
                        vect_req.remove(0);
                    }
                    if asked.len() == 1{
                        return Err(String::from("Nothing selected"));
                    }
                    asked.pop();
                    arguments.insert(String::from(":asked"), asked);
                }
            }
            if vect_req.len() > 0{
                let from_keyword = vect_req.remove(0);
                match from_keyword{
                    "FROM" => {
                        let table_name = vect_req.remove(0);
                        if self.is_correct_name(table_name){
                            arguments.insert(String::from(":table_name"), table_name.to_string());
                            if vect_req.len() > 0{
                                let keyword = vect_req.remove(0);
                                match keyword{
                                    "WHERE" => {
                                        if vect_req.len() == 0{
                                            return Err(String::from("Error: Condition missing."));
                                        }else{
                                            let condition = vect_req.join(" ");
                                            let cleaning = self.clean_the_condition(condition, &mut arguments);
                                            match cleaning{
                                                Some(condition) => {
                                                    arguments.insert(String::from(":condition"), condition);
                                                    self.convert_in_str_hashmap(&arguments, &mut result);
                                                    return self.system.new_request(result);
                                                }
                                                None => return Err(String::from("The condition doesn't respect the syntax rules.")),
                                            }
                                        }
                                    }
                                    _ => return Err(format!("Bad keyword: {}", keyword))
                                }
                            }else{
                                arguments.insert(String::from(":condition"), String::from("1 == 1"));
                                self.convert_in_str_hashmap(&arguments, &mut result);
                                return self.system.new_request(result);
                            }
                        }else{
                            return Err(format!("{} isn't a correct name for a table.", table_name));
                        }
                    },   
                    _ => return Err(format!("{} found where From was expected", from_keyword)),
                }
            }
            return Err(String::from("Nothing found where FROM was expected."))
        }
        Err(format!("The request 'SELECT {}' isn't valid.", vect_req.join(" ")))
    }


    fn delete_line(&mut self, mut vect_req: Vec::<&str>) -> Result<(), String>{
        if vect_req.len() >= 2{
            let from_keyword = vect_req.remove(0);
            if from_keyword == "FROM"{
                if vect_req[0].len() > 2 && self.is_correct_name(&vect_req[0]){
                    let table_name = vect_req.remove(0);
                    let mut arguments = HashMap::<String, String>::new();
                    let mut result = HashMap::<&str, &str>::new();
                    arguments.insert(String::from(":table_name"), table_name.to_string());
                    arguments.insert(String::from(":request"), String::from("DELETE_LINE_IF"));
                    if vect_req.len() == 0{
                        arguments.insert(String::from(":condition"), String::from("1 == 1"));
                        self.convert_in_str_hashmap(&arguments, &mut result);
                        self.system.new_request(result)?;
                    }else{
                        let key_word = vect_req.remove(0); 
                        match key_word{
                            "WHERE" => {
                                if vect_req.len() == 0{
                                    return Err(String::from("Condition missing"));
                                }else{
                                    let condition = vect_req.join(" ");
                                    let cleaning = self.clean_the_condition(condition, &mut arguments);
                                    match cleaning{
                                        Some(condition) => {
                                            arguments.insert(String::from(":condition"), condition);
                                            self.convert_in_str_hashmap(&arguments, &mut result);
                                            self.system.new_request(result)?;
                                        }
                                        None => return Err(String::from("The condition doesn't respect the syntax rules."))
                                    }
                                }
                            }   
                            _ => {
                                return Err(format!("Bad key_word here: {}", key_word));
                            }
                        }
                    }
                }else{
                    if vect_req[0].len() <= 2{
                        return Err(String::from("Nothing found when a table name was expected."));
                    }else{
                        return Err(format!("'{}' isn't a correct name for a table.", vect_req[0]));
                    }
                }
            }else{
                return Err(format!("'{}' found whene FROM was expected", from_keyword));
            }
        }else {
            return Err(format!("The request 'DELETE {}' isn't valid.", vect_req.join(" ")));
        }
        Ok(())
    }

    fn replace_the_space(&self, cond: String, operator: &str, ok_to_replace: bool) -> Option<String>{
        let mut split: Vec::<String> = cond.split(operator).map(String::from).collect();
        for i in 0..split.len(){
            if i != 0 && split[i].chars().next() != Some(' '){
                if !ok_to_replace{
                    return None;
                }
                split[i] = String::from(" ") + &split[i];
            } 
            if i != split.len()-1 && split[i].chars().rev().next() != Some(' '){
                if !ok_to_replace{
                    return None;
                }
                split[i] += " ";
            }
        }
        Some(split.join(operator))
    }

    fn clean_the_condition(&self, mut cond: String, arg: &mut HashMap::<String, String>) -> Option<String>{
        let _ = cond.replace("  ", " ");

        match self.replace_the_space(cond, "AND", false){
            Some(new_cond) => cond = new_cond,
            None => return None
        }

        match self.replace_the_space(cond, "OR", false){
            Some(new_cond) => cond = new_cond,
            None => return None
        }

        for i in 0..7{
            cond = self.replace_the_space(cond, self.type_gestion.get_nth_operator(i), true).unwrap();
        }
        cond = self.replace_the_space(cond, ")", true).unwrap();

        let mut space_split: Vec<String> = cond.split_whitespace().map(String::from).collect();
        let mut i = 0;
        let n = space_split.len() - 1;
        let mut opening = 0;
        let mut ending = 0;
        let par = String::from("()");
        while i<space_split.len(){
            if self.type_gestion.operator_exist(&space_split[i]) && space_split[i] != "("{
                if i == 0 || i == n || (self.type_gestion.operator_exist(&space_split[i-1]) && space_split[i-1] != "(") || (self.type_gestion.operator_exist(&space_split[i+1]) && space_split[i+1] != "("){
                    return None;
                }  
            }else if space_split[i] == "("{
                opening += 1;
                if (i == n || (i != 0 && !self.type_gestion.operator_exist(&space_split[i-1])) && !par.contains(&space_split[i-1])) || (self.type_gestion.operator_exist(&space_split[i+1]) && !par.contains(&space_split[i+1])){
                    return None;
                }
            }else if space_split[i] == ")"{
                ending += 1;
                if ending > opening || (self.type_gestion.operator_exist(&space_split[i-1]) && !par.contains(&space_split[i-1])) || (i != n && (!self.type_gestion.operator_exist(&space_split[i+1]) && !par.contains(&space_split[i+1]))){
                    return None;
                }
            }else{
                if (i != 0 && !self.type_gestion.operator_exist(&space_split[i-1])) || (i != n && !self.type_gestion.operator_exist(&space_split[i+1]) && space_split[i+1] != ")" && space_split[i].to_string().chars().next().unwrap() != '\''){
                    return None;
                } 
                if !self.type_gestion.is_float(&space_split[i]){
                    if space_split[i] == String::from("true"){
                        space_split[i] = String::from("1");
                    }else if space_split[i] == String::from("false"){
                        space_split[i] = String::from("0");
                    }else if space_split[i].chars().next() == Some('\''){
                        space_split[i].remove(0);
                        let mut the_string = String::from(space_split.remove(i));
                        while i != space_split.len() && the_string.chars().rev().next().unwrap() != '\''{
                            the_string += &space_split.remove(i);
                        }
                        if the_string.pop() != Some('\''){
                            return None;
                        }
                        space_split.insert(i, format!("{}", self.type_gestion.hash_string_to_number(the_string)));
                    }else{
                        if space_split[i].contains("."){
                            let p_split: Vec::<&str> = space_split[i].split(".").collect();
                            if p_split.len() != 2 || !self.is_correct_name(p_split[0]) || !self.is_correct_name(p_split[1]){
                                return None;
                            }
                            arg.insert(p_split[1].to_string(), p_split[0].to_string());
                        }else if !self.is_correct_name(&space_split[i]){
                            return None;
                        }else{
                            arg.insert(space_split[i].to_string(), String::from(""));
                        }
                    }
                }
            }
            i += 1;
        }
        if ending != opening{
            return None;
        }
        Some(space_split.join(" "))
    }   


    
    fn insert_request(&mut self, mut vect_req: Vec::<&str>) -> Result<(), String>{
        let mut arguments = HashMap::<String, String>::new();
        let mut result = HashMap::<&str, &str>::new();
        let table_name = vect_req.remove(0);
        result.insert(":request", "INSERT");
        if self.is_correct_name(&table_name){
            arguments.insert(":table_name".to_string(), table_name.to_string());
            let mut req = vect_req.join(" ");
            req = req.replace(", ", ",");
            req = req.replace(" ,", ",");
            let split_req_value: Vec<&str> = req.split(" VALUES ").collect();
            if split_req_value.len() == 2{
                let mut arg_s = split_req_value[0].to_string();
                let mut values_s = split_req_value[1].to_string();
                if arg_s.remove(0) == '(' && arg_s.pop() == Some(')') && values_s.remove(0) == '(' && values_s.pop() == Some(')'){
                    let mut values: Vec<String> = values_s.split(",").map(String::from).collect();
                    let args: Vec<String> = arg_s.split(",").map(String::from).collect();
                    if values.len() == args.len(){
                        for i in 0..values.len(){
                            if self.is_correct_name(&args[i]){
                                if values[i].chars().next() == Some('\'') && values[i].remove(0) == '\'' && values[i].pop() != Some('\''){
                                    return Err(String::from("You forgot to close this: '"));
                                }else{
                                    arguments.insert(args[i].to_string(), values[i].to_string());
                                }
                            }else{
                                return Err(format!("This name isn't correct for a variable: {}", args[i]));
                            }
                        }
                        
                        self.convert_in_str_hashmap(&arguments, &mut result);
                        self.system.new_request(result)?;
                    }else{
                        return Err(String::from("It seems like the number of values is different then the number of arguments"));
                    }
                }else{
                    return Err(String::from("A parenthésis is missing in your request."));
                }
            }else{
                if split_req_value.len() < 2{
                    return Err(String::from("The 'VALUES' keyword is missing."));
                }else{
                    return Err(String::from("The 'VALUES' keyword was found on two occasions."));
                }
            }
        }else{
            return Err(format!("The name {} isn't valid for a table", table_name));
        }
        Ok(())
    }

    fn create_req(&mut self, mut vect_req: Vec::<&str>) -> Result<(), String>{
        if vect_req.len() >= 2{
            let thing_to_create = vect_req.remove(0);
            match thing_to_create{
                "TABLE" => {
                    let mut new_table = String::from(vect_req.join(" "));
                    if new_table.pop() != Some(')'){
                        return Err(String::from("A parenthésis is missing in your request."));
                    }else{
                        _ = new_table.replace(", ", ",");
                        let mut splited_req_for_name: Vec::<&str> = new_table.split("(").collect();
                        let table_name = splited_req_for_name.remove(0);
                        if self.is_correct_name(table_name) && splited_req_for_name.len() >= 2{
                            let arg_string = splited_req_for_name.join("(");
                            let virgule_split: Vec::<&str> = arg_string.split(",").collect();
                            let mut arguments = HashMap::<String, String>::new();
                            let mut p_key = false;
                            arguments.insert(":request".to_string(), "CREATE".to_string());
                            arguments.insert(":table_name".to_string(), table_name.to_string());
                            for arg in virgule_split{
                                let mut splited_arg: Vec::<&str> = arg.split_whitespace().collect();
                                let column_name = splited_arg.remove(0);
                                let type_data = splited_arg.remove(0);
                                let mut bonus_param = String::new();
                                if self.is_correct_name(column_name) && self.type_gestion.is_correct_type(type_data){
                                    if splited_arg.len() > 0{
                                        let mut other = splited_arg.join(" ");
                                        match &other[..]{
                                            "PRIMARY KEY" => {
                                                if !p_key{
                                                    p_key = true;
                                                    arguments.insert(":primary".to_string(), String::from(column_name));
                                                    bonus_param = "NOTNULL".to_string();
                                                }else{
                                                    return Err(format!("It seems that you have declared two primary keys, so '{}' will not be primary key.", column_name));
                                                } 
                                            }
                                            "FOREIGN KEY" => {
                                                bonus_param = "FOREIGN".to_string();
                                            }
                                            "NOT NULL" => {
                                                bonus_param = "NOTNULL".to_string();
                                            }
                                            _ => {
                                                if other.starts_with("DEFAULT "){
                                                    for _ in 0..8{
                                                        other.remove(0);
                                                    }
                                                    if self.type_gestion.good_type_and_good_value(type_data, &other){
                                                        bonus_param = String::from("DEFAULT");
                                                        arguments.insert(format!("${}", String::from(column_name)), other);
                                                    }else{
                                                        return Err(format!("{} isn't an appropriate value for the type {}.", other, type_data));
                                                    }
                                                }else{
                                                    return Err(format!("{} is unknow by the system.", other));
                                                }
                                            }
                                        }   
                                    }
                                    arguments.insert(String::from(column_name), type_data.to_string()+" "+&bonus_param);    
                                }else if !self.type_gestion.is_correct_type(type_data){
                                    return Err(format!("The type {} isn't correct.", type_data));
                                }else{
                                    return Err(format!("The name {} isn't correct.", column_name));
                                }
                                
                            }
                            if p_key{
                                let mut result = HashMap::<&str, &str>::new();
                                self.convert_in_str_hashmap(&arguments, &mut result);
                                self.system.new_request(result)?;
                                Ok(())
                            }else{
                                return Err(String::from("Primary key is missing."));
                            }
                        }else{
                            if splited_req_for_name.len() < 2{
                                return Err(String::from("You have to specify the column of a new table between parenthesis: (column1 type, column2 type ...)"));
                            }
                            return Err(format!("{} isn't a correct name for a table.", table_name));
                        }
                    }
                }
                _ => {
                    return Err(format!("{} is unknow by the system.", thing_to_create));
                }
            }
        }else{
            return Err(format!("CREATE {} n'est pas une commande valide", vect_req.join(" ")));
        }
    }

    fn is_correct_name(&self, name: &str) -> bool{
        for letter in name.chars(){
            if !self.authorized_char_for_variable.contains(letter){
                return false;
            }
        }
        true
    }

    fn convert_in_str_hashmap<'a>(&self, hashmap_to_convert: &'a HashMap<String, String>, result: &mut HashMap<&'a str, &'a str>) {
        for (cle, value) in hashmap_to_convert {
            let cle_str: &'a str = cle.as_str();
            let value_str: &'a str = value.as_str();
            result.insert(cle_str, value_str);
        }
    }
    
}
