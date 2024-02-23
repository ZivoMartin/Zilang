use crate::text_file::{src_path, file_exists, TextFile};
use crate::type_gestion::TypeGestion;
use std::collections::HashMap;
use std::fs::read_dir;

pub struct System{
    main_file: TextFile,
    type_gestion: TypeGestion
}

impl System{

    pub fn new() -> System{
        System{
            main_file: TextFile::new(String::from("text_files/main_file.txt")), 
            type_gestion: TypeGestion::new()
        }
    }

    pub fn new_request(&mut self, mut args: HashMap<String, String>) -> Result<Option<HashMap<String, Vec<String>>>, String>{  
        let type_request = args.remove(":request").unwrap();
        match &type_request as &str{
            "CREATE" => self.create_table(args)?,
            "INSERT" => self.insert_line(args)?,
            "DELETE_LINE" => self.delete_line(&args[":table_name"], &args[":primary"])?,
            "DELETE_TABLE" => self.delete_table(&args[":table_name"])?,
            "DELETE_LINE_IF" => {self.browse_lines(args, "delete")?;},
            "SELECT" => {
                match self.browse_lines(args, "select"){
                    Ok(res) => return Ok(Some(res)),
                    Err(e) => return Err(e.to_string())
                }
            }
            "RESET" => self.reset_database(),
            "UPDATE" => self.update_database(args)?,
            _ => return Err(format!("The keyword {} isn't valid.", type_request))
        }
        Ok(None)
    }

    fn reset_database(&self) {
        let reader = read_dir(src_path() + "text_files").unwrap().map(|p| p.unwrap());
        for file in reader{
            let file_name = file.file_name().into_string().unwrap();
            if file_name == "main_file.txt" {
                TextFile::new("text_files/".to_string() + &file_name).reset("");
            }else{
                TextFile::new("text_files/".to_string() + &file_name).erase();
            }
        }
    }

    fn update_database(&mut self, mut args: HashMap<String, String>) -> Result<(), String>{
        let table_name = args.remove(":table_name").unwrap();
        self.table_exists(&table_name)?;
        let binding_cols = args.remove(":cols").unwrap(); 
        let mut cols = binding_cols.split("/");
        let binding_val = args.remove(":values").unwrap();
        let mut values = binding_val.split("/");
        while let Some(_value) = values.next() {
            if let Some(_col) = cols.next() {
                
            }else{
                panic!("not the same number of col and value");
            }
        }   
        Ok(())
    }
    


    fn browse_lines(&mut self, mut arg: HashMap::<String, String>, id: &str) -> Result<HashMap<String, Vec<String>>, String>{
        let table_name = arg.remove(":table_name").unwrap();
        let condition = arg.remove(":condition").unwrap();
        let mut string_hashmap = HashMap::<String, String>::new();
        self.table_exists(&table_name)?;
        let mut result = HashMap::<String, Vec<String>>::new();
        let mut table_file = TextFile::new(format!("text_files/{}", table_name));
        let mut data_file = TextFile::new(format!("text_files/data_{}", table_name));
        let mut asked_hash_map = HashMap::<String, i32>::new();  
        let asked_string = arg.remove(":asked").unwrap();
        if asked_string == String::from("*") {
            if id != "select" {
                return Err(String::from("Invalid character: *"));
            }
            let txt_data = data_file.get_text();
            let split = txt_data.split("\n");
            let mut i = 0;
            for elt in split{
                if elt != ""{
                    let column = String::from(elt.split_whitespace().nth(0).unwrap());
                    asked_hash_map.insert(column.clone(), i);
                    result.insert(column, Vec::<String>::new());
                }
                i += 1;
            }
        }else if !asked_string.is_empty(){
            let asked_string_split = asked_string.split("/");
            for ask in asked_string_split{
                match self.get_arg_data(&mut data_file, &ask){
                    Some(res) => {
                        asked_hash_map.insert(ask.to_string(), res.0);
                        result.insert(ask.to_string(), Vec::<String>::new());
                    }
                    None => return Err(format!("The column {} doesn't exist for the table {}", ask, table_name))
                }
            }
        }
        let mut text = table_file.get_text();
        if !text.is_empty(){
            text.pop();
            let mut data_text = data_file.get_text();
            if !data_text.is_empty() {
                data_text.pop();
            }
            let data_text_splited: Vec<&str> = data_text.split("\n").collect();
            let keys: Vec::<String> = arg.keys().cloned().collect();
            for p_key in text.split("\n"){
                let mut line_file = TextFile::new(format!("text_files/{}_line_{}", table_name, p_key));
                let line_file_text = line_file.get_text();
                let line_text_split: Vec::<&str> = line_file_text.split("\n").collect();
                for i in 0..data_text_splited.len(){
                    let split_space: Vec<String> = data_text_splited[i].split_whitespace().map(String::from).collect();
                    if keys.contains(&split_space[0]){
                        let mut arg_data = self.get_good_data(line_text_split[i].to_string());
                        if split_space[1].starts_with("VARCHAR"){
                            arg_data = format!("{}", self.type_gestion.hash_string_to_number(arg_data));
                        }else if split_space[1] == "BOOL"{
                            arg_data = self.type_gestion.convert_bool_to_number(&arg_data);
                        }
                        string_hashmap.insert(split_space[0].clone(), arg_data);
                    }
                }
                let bool_string_for_this_line = self.build_bool_string(condition.to_string(), &string_hashmap);
                if self.type_gestion.descript_a_string_bool(&bool_string_for_this_line){
                    match id {
                        "delete" => self.delete_line(&table_name, &p_key)?,
                        "select" => {
                            for (key, value) in &asked_hash_map{
                                result.get_mut(key).unwrap().push(self.get_good_data(line_text_split[*value as usize].to_string()));
                            }
                        }
                        _ => panic!("tried to brows with {}", id)
                    }
                }
            }
        }
        return Ok(result)
    }


    fn build_bool_string(&self, bool_string: String, arg: &HashMap::<String, String>) -> String{
        let keys: Vec::<String> = arg.keys().cloned().collect();
        let mut split: Vec::<&str> = bool_string.split_whitespace().collect();
        for i in 0..split.len(){
            if keys.contains(&split[i].to_string()){
                split[i] = &arg[split[i]];
            }
        }  
        split.join(" ")
    }

    fn create_table(&mut self, mut arg: HashMap<String, String>) -> Result<(), String>{
        let new_table_name = arg.remove(":table_name").unwrap();
        let new_table_path = format!("text_files/{}", new_table_name);
        let new_table_data_path = format!("text_files/data_{}", new_table_name);
        if !self.table_exists(&new_table_name).is_ok(){
            self.main_file.push(&format!("{}\n", new_table_name));
            TextFile::new(new_table_path);
            let mut new_table_data_file = TextFile::new(new_table_data_path);
            let primary_key = arg.remove(":primary").unwrap();
            let mut data_text = format!("{} {}\n", primary_key, arg.remove(&primary_key).unwrap());
            for (var_name, type_var) in &arg{
                if !var_name.starts_with("$") && !var_name.starts_with(":"){
                    let split_type: Vec::<&str> = type_var.split_whitespace().collect();
                    match split_type.len(){
                        1 => data_text += &format!("{} {}\n", &var_name, split_type[0]),
                        2 => {
                            match split_type[1]{
                                "DEFAULT" => data_text += &format!("{} {} DEFAULT {}\n", var_name, split_type[0], arg[&format!("${}", &var_name) as &str]),
                                _ => return Err(format!("{} keyword is unvalid here", split_type[1]))
                            }
                        },
                        _ => return Err(format!("{} isn't a valid parameter for a column.", split_type.join(" ")))
                    }
                }
            }
            new_table_data_file.push(&data_text);
            return Ok(());
        }
        return Err(format!("The table {} already exist.", new_table_name))
    }

    fn insert_line(&mut self, mut arg: HashMap<String, String>) -> Result<(), String>{
        let name = arg.remove(":table_name").unwrap();
        self.table_exists(&name)?;
        let mut tab_file = TextFile::new(format!("text_files/{}", name));
        let mut data_file = TextFile::new(format!("text_files/data_{}", name));
        let p_key = self.get_primary_key(&name);
        let p_key_val_result = arg.remove(&p_key as &str);
        match p_key_val_result{
            Some(p_key_val) => {
                let line_file_name = format!("text_files/{}_line_{}", name, p_key_val);
                let text = data_file.get_text();
                let mut line_text = format!("d{}\n", p_key_val);
                for line in text.lines().skip(1) {
                    let mut splited_line = line.split_whitespace();
                    let data_name = splited_line.nth(0).unwrap();
                    let mut data_type = splited_line.nth(0).unwrap();
                    if data_type.starts_with("VARCHAR"){
                        data_type = "STRING"
                    }
                    match splited_line.nth(0){
                        Some(next_word) => {
                            match next_word{
                                "DEFAULT" => line_text = self.push_new_txt(line_text, &splited_line.nth(0).unwrap(), &format!("{}_{}", line_file_name, data_name)),
                                "NOTNULL" => {
                                    let arg_in_request = arg.remove(data_name);
                                    match arg_in_request{
                                        Some(val) => {
                                            if !self.type_gestion.good_type_and_good_value(&data_type, &val){
                                                return Err(format!("The type {} wasn't expected.", val))
                                            } 
                                            line_text = self.push_new_txt(line_text, &val, &format!("{}_{}", line_file_name, data_name));
                                        },
                                        None =>  {
                                            return Err(format!("The column {} was defined with the NOT NULL parameter !", data_name));
                                        }                               
                                    }   
                                }_ =>{}
                            }
                        },
                        None => {
                            let arg_in_request = arg.remove(data_name);
                            match arg_in_request{
                                Some(val) => {
                                    if !self.type_gestion.good_type_and_good_value(&data_type, &val){
                                        return Err(format!("The type {} wasn't expected.", val))
                                    }
                                    line_text = self.push_new_txt(line_text, &val, &format!("{}_{}", line_file_name, data_name));
                                }
                                None =>  line_text = self.push_new_txt(line_text, "NULL", &format!("{}_{}", line_file_name, data_name))
                            }
                            
                            
                        } 
                    }
                }
                if !file_exists(&line_file_name){
                    let mut p_key_val_s = p_key_val.to_string();
                    p_key_val_s.push_str("\n");
                    tab_file.push(&p_key_val_s);
                    let mut line_file = TextFile::new(line_file_name);
                    line_file.push(&line_text);
                    return Ok(());
                }else{
                    return Err(format!("The line with the primary key {} already exists", p_key));
                }
            }None => {
                return Err(String::from("Primary key missing."));
            }
        }
    }

    
    fn get_primary_key(&self, table_name: &str) -> String{
        TextFile::new(format!("text_files/data_{}", table_name)).get_text().split("\n").nth(0).unwrap().split_whitespace().nth(0).unwrap().to_string()
    }

    fn push_new_txt(&self, mut txt1: String, txt2: &str, potential_path: &str) -> String{
        if txt2.contains("\n"){
            txt1.push_str(&format!("f{}\n", potential_path));
            let mut file = TextFile::new(String::from(potential_path));
            file.push(txt2);
        }else{
            txt1.push_str(&format!("d{}\n", txt2));
        }
        txt1
    }

    /// Return an error if the table doesn't exists
    fn table_exists(&mut self, tab_name: &str) -> Result<(), String>{
        let text = self.main_file.get_text();
        for line in text.lines() {
            if line.split_whitespace().next() == Some(tab_name) {
                return Ok(()) 
            }
        }
        Err(format!("the table {} doesn't exists", tab_name))
    }
    

    

    fn delete_line(&mut self, table_name: &str, primary_key: &str) -> Result<(), String>{
        let line_file_path = format!("text_files/{}_line_{}", table_name, primary_key);
        self.table_exists(table_name)?;
        if file_exists(&line_file_path){
            let mut table_file = TextFile::new(format!("text_files/{}", table_name));
            table_file.replace(&format!("{}\n", primary_key), "");                                                                                                                                                                                                                                                                                                      
            let line_file = TextFile::new(line_file_path);
            self.clear_line_file(line_file);
            Ok(())
        }else{
            Err(format!("The line {} don't already exists.", line_file_path.split("/").last().unwrap()))
        }
    }

    fn clear_line_file(&self, mut line_file: TextFile){
        let text = line_file.get_text();
        for line in text.lines(){
            let mut s_line = String::from(line);
            if s_line[0..1] == String::from("f") {
                s_line.remove(0);
                TextFile::new(s_line).erase();
            }
        }
        line_file.erase();
    }

    fn get_good_data(&self, mut value: String) -> String{
        let type_data = value.remove(0);
        match type_data{
            'f' => return TextFile::new(value).get_text(),
            _ => return value
        }
    }

    fn get_arg_data(&self, data_file: &mut TextFile, arg: &str) -> Option<(i32, String)>{
        let text = data_file.get_text();
        for (i, line) in text.lines().enumerate(){
            let mut splited_line = line.split_whitespace();
            let name = splited_line.nth(0).unwrap();
            if name == arg {
                return Some((i as i32, line.split_whitespace().nth(0).unwrap().to_string()));
            }
        }
        return None;
    }   

    fn delete_table(&mut self, table_name: &str) -> Result<(), String>{
        self.table_exists(table_name)?;
        let mut tab_file = TextFile::new(format!("text_files/{}", table_name));
        let text = tab_file.get_text();
        for line in text.lines(){
            self.delete_line(table_name, line)?;
        }
        tab_file.erase();
        TextFile::new(format!("text_files/data_{}", table_name)).erase();
        self.main_file.replace(&format!("{}\n", table_name), "");
        return Ok(());
    }
}


