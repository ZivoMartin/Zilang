
use crate::system::System;
use std::collections::HashMap;
use crate::type_gestion::TypeGestion;
use std::fs::File;
use std::process::exit;
pub struct Interpreteur {
    system: System,
    authorized_char_for_variable: &'static str,
    type_gestion: TypeGestion
}

pub struct ResponseData {
    json_path: String,
    pretty: bool
}

impl ResponseData {
    pub fn new(json_path: String, pretty: bool) -> ResponseData{
        ResponseData{json_path, pretty}
    }

    pub fn new_empty() -> ResponseData {
        ResponseData{json_path: String::new(), pretty: false}
    }
}

impl Interpreteur {
    pub fn new() -> Interpreteur{
        Interpreteur{
            system: System::new(),
            authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
            type_gestion: TypeGestion::new()
        }
    }

    pub fn sqlrequest(&mut self, mut req: String, response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if req != "" {
            req = req.replace(",", " , ");
            while req.contains("  ") {
                req = req.replace("  ", " ");
            }
            let mut vect_req: Vec<&str> = req.split(" ").map(str::trim).collect();
            let type_request = vect_req.remove(0);
            return match type_request{
                "DROP" => self.drop_req(vect_req, response_data),
                "CREATE" => self.create_req(vect_req, response_data),
                "INSERT" => self.insert_request(vect_req, response_data),
                "DELETE" => self.delete_line(vect_req, response_data),
                "SELECT" => self.select_request(vect_req, response_data),
                "RESET" => self.reset_request(vect_req, response_data),
                "UPDATE" => self.update_request(vect_req, response_data),
                _ => return Err(format!("{} is unnknow by the system.", type_request))
            }
        }
        Ok(None)
    }
    

    fn update_request(&mut self, mut vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 3 {
            let mut arguments = HashMap::<String, String>::new();
            arguments.insert(String::from(":request"), String::from("UPDATE"));
            self.valid_table_name(&mut arguments, &mut vect_req)?;
            let mut cols = String::new();
            let mut values = String::new();
            let keyword = vect_req.remove(0);
            match keyword {
                "SET" => {
                    while vect_req.len() > 0{
                        let exp = vect_req.remove(0);
                        if exp.matches('=').count() == 1 {
                            let mut split = exp.split("=");
                            cols += &(split.next().unwrap().trim().to_string() + "/");
                            values += &(split.next().unwrap().trim().to_string() + "/");
                        }else{
                            if !cols.is_empty(){
                                cols.pop();
                            }
                            if !values.is_empty(){
                                values.pop();
                            }
                            break;
                        }                    
                    }
                    arguments.insert(String::from(":cols"), cols);
                    arguments.insert(String::from(":values"), values);
                    self.catch_condition(&mut arguments, vect_req)?;
                    return self.system.new_request(arguments)
                },
                _ => return Err(format!("The keyword {} is unknow by the system.", keyword))
            }
        }
        Err(format!("Not enough token for the request UPDATE {}", vect_req.join(" ")))
    }

    fn reset_request(&mut self, _vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        let mut arguments = HashMap::<String, String>::new();
        arguments.insert(":request".to_string(), "RESET".to_string());
        self.system.new_request(arguments)
    }

    fn drop_req(&mut self, vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 2{
            let mut arguments = HashMap::<String, String>::new();
            arguments.insert(":request".to_string(), "DELETE_TABLE".to_string());
            match vect_req[0]{
                "TABLE" => {
                    for table_to_drop in vect_req.iter().skip(1){
                        arguments.insert(":table_name".to_string(), table_to_drop.to_string()); // We insert the table to drop
                        self.system.new_request(arguments.clone())?;    // We drop it
                    }
                }
                _ => return Err(format!("{} is unknow by the system", vect_req[0]))
            }
            return Ok(None)
        }
        Err(format!("DROP {} isn't a valid command.", vect_req.join(" ")))
    }



    fn select_request(&mut self, mut vect_req: Vec<&str>, response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 3 {
            let mut arguments = HashMap::<String, String>::new();
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
                        self.valid_table_name(&mut arguments, &mut vect_req)?;
                        self.catch_condition(&mut arguments, vect_req)?;
                        let result = self.system.new_request(arguments);
                        if result.is_ok() && !response_data.json_path.is_empty() {
                            let json_file = File::create(response_data.json_path).unwrap_or_else(|e| {
                                eprintln!("{e}");
                                exit(1);
                            });
                            if response_data.pretty {
                                serde_json::to_writer_pretty(json_file, &result.clone().unwrap()).unwrap_or_else(|e| {
                                    eprintln!("{e}");
                                    exit(1);
                                });
                            }else{
                                serde_json::to_writer(json_file, &result.clone().unwrap()).unwrap_or_else(|e| {
                                    eprintln!("{e}");
                                    exit(1);
                                });
                            }
                            
                        }
                        return result
                    },   
                    _ => return Err(format!("{} found where From was expected", from_keyword)),
                }
            }
            return Err(String::from("Nothing found where FROM was expected."))
        }
        Err(format!("The request 'SELECT {}' isn't valid.", vect_req.join(" ")))
    }


    fn delete_line(&mut self, mut vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 2{
            let from_keyword = vect_req.remove(0);
            if from_keyword == "FROM"{
                if vect_req[0].len() > 2 && self.is_correct_name(&vect_req[0]){
                    let mut arguments = HashMap::<String, String>::new();
                    arguments.insert(":asked".to_string(), String::new());
                    self.valid_table_name(&mut arguments, &mut vect_req)?;
                    arguments.insert(String::from(":request"), String::from("DELETE_LINE_IF"));
                    self.catch_condition(&mut arguments, vect_req)?;
                    return self.system.new_request(arguments)
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
    }

    fn catch_condition(&mut self, mut arguments: &mut HashMap<String, String>, mut vect_req: Vec<&str>) -> Result<(), String> {
        if vect_req.len() > 0{
            let keyword = vect_req.remove(0);
            match keyword{
                "WHERE" => {
                    if vect_req.is_empty() {
                        return Err(String::from("Condition missing"));
                    }
                    let condition = vect_req.join(" ");
                    let cleaning = self.clean_the_condition(condition, &mut arguments);
                    match cleaning{
                        Some(condition) => arguments.insert(String::from(":condition"), condition),
                        _ => return Err(String::from("The condition doesn't respect the syntax rules."))
                    };
                },
                _ => return Err(format!("Bad keyword: {}", keyword))
            }
        }else{
            arguments.insert(String::from(":condition"), String::from("1 == 1"));
        }
        Ok(())
    }

    fn valid_table_name(&mut self, arguments: &mut HashMap<String, String>, vect_req: &mut Vec::<&str>) -> Result<(), String> {
        let table_name = vect_req.remove(0);
        if self.is_correct_name(table_name){
            arguments.insert(String::from(":table_name"), table_name.to_string());
        }else{
            return Err(format!("{} isn't a correct name for a table.", table_name));
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


    
    fn insert_request(&mut self, mut vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() == 0 || vect_req.remove(0) != "INTO" {
            return Err(String::from("Invalid syntax for a INSERT request"))
        }
        let mut arguments = HashMap::<String, String>::new();
        self.valid_table_name(&mut arguments, &mut vect_req)?;
        arguments.insert(String::from(":request"), String::from("INSERT"));
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
                    
                    return self.system.new_request(arguments)
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
    }

    fn create_req(&mut self, mut vect_req: Vec::<&str>, _response_data: ResponseData) -> Result<Option<HashMap<String, Vec<String>>>, String>{
        if vect_req.len() >= 2{
            let thing_to_create = vect_req.remove(0);
            match thing_to_create{
                "TABLE" => {
                    let mut new_table = String::from(vect_req.join(" "));
                    if new_table.pop() != Some(')'){
                        return Err(String::from("A parenthésis is missing in your request."));
                    }else{
                        _ = new_table.replace(", ", ",");
                        let mut arguments = HashMap::<String, String>::new();
                        let mut splited_req_for_name: Vec::<&str> = new_table.split("(").collect();
                        self.valid_table_name(&mut arguments, &mut splited_req_for_name)?;
                        if splited_req_for_name.len() >= 2{
                            let arg_string = splited_req_for_name.join("(");
                            let virgule_split: Vec::<&str> = arg_string.split(",").collect();
                            let mut p_key = false;
                            arguments.insert(":request".to_string(), "CREATE".to_string());
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
                                            "FOREIGN KEY" => bonus_param = "FOREIGN".to_string(),
                                            "NOT NULL" => bonus_param = "NOTNULL".to_string(),
                                            _ => {
                                                if other.starts_with("DEFAULT "){
                                                    other = String::from(&other[8..other.len()]);
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
                                return self.system.new_request(arguments)
                            }
                            return Err(String::from("Primary key is missing."));
                        }
                        return Err(String::from("You have to specify the column of a new table between parenthesis: (column1 type, column2 type ...)"));
                    }
                }
                _ => {
                    return Err(format!("{} is unknow by the system.", thing_to_create));
                }
            }
        }
        Err(format!("CREATE {} n'est pas une commande valide", vect_req.join(" ")))
    }

    fn is_correct_name(&self, name: &str) -> bool{
        for letter in name.chars(){
            if !self.authorized_char_for_variable.contains(letter){
                return false;
            }
        }
        true
    }
    
}
