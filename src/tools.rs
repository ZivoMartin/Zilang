
#[allow(dead_code)]
pub mod tools{
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use std::path::PathBuf;
    use std::io::BufRead;
    use std::io::Seek;

    pub struct TextFile{
        file_path: PathBuf,
        file: File
    }
    impl TextFile{

        pub fn new(file_path: String) -> Result<TextFile, String> {
            if !file_exists(&file_path) {
                create_file(&file_path);
            }

            let file = match fs::OpenOptions::new().append(true).read(true).open(&file_path) {
                Ok(file) => file,
                Err(error) => return Err(format!("Error opening file: {}", error)),
            };

            Ok(TextFile {
                file_path: PathBuf::from(&file_path),
                file,
            })
        }

        pub fn push(&mut self, text: &str){
            self.file.write_all(text.as_bytes())
            .unwrap_or_else(|e|{
                println!("L'ajout du texte a la fin du fichier a echoué: {}", e);
            });
        }

        pub fn reset(&mut self, new_text: &str){
            self.file.set_len(0)
            .unwrap_or_else(|e|{
                println!("Le reset du texte a echoué: {}", e);
            });
            self.push(new_text);
        }

        pub fn erase(&self){
            fs::remove_file(&self.file_path)
            .unwrap_or_else(|e| {
                println!("Le fichier n'a pas été supprimé: {}", e);
            });
        }


        pub fn get_text(&mut self) -> String {
            let _ = self.file.seek(std::io::SeekFrom::Start(0));
            let mut result = String::new();
            let lines = io::BufReader::new(&self.file).lines();
            for line in lines {
                match line {
                    Ok(the_line) => {
                        result.push_str(&the_line);
                        result.push_str("\n");
                    }Err(e) => {
                        println!("Erreur lors de la lecture de la ligne {}", e);
                        return result;
                    }
                }
            }
            result
        }

        pub fn replace(&mut self, text_to_replace: &str, new_text: &str){
            let new_txt = self.get_text().replace(text_to_replace, new_text);
            self.reset(&new_txt);
        }
    }


    pub fn file_exists(file_path: &str) -> bool {
        fs::metadata(file_path).is_ok()
    }

    fn create_file(file_path: &str){
        let _ = File::create(&file_path).map_err(|e|{
            println!("Erreur lors de la creation du fichier {}: {}", file_path, e);
        });
    }


    pub struct Tools{
        authorized_type: Vec<&'static str>,
        operator_list: Vec<&'static str>,
        authorized_char_for_variable: &'static str
    }

    impl Tools{

        pub fn new() -> Tools{
            Tools{
                authorized_type: vec!{"INT"},
                operator_list: vec!{"+"},
                authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_"
            }
        }

        pub fn is_correct_name(&self, name: &str) -> bool{
            for letter in name.chars(){
                if !self.authorized_char_for_variable.contains(letter){
                    return false;
                }
            }
            true
        }

        pub fn get_nth_operator(&self, n: i32)->&str{
            self.operator_list[n as usize]
        }

        pub fn operator_exist(&self, op: &str)->bool{
            self.operator_list.contains(&op)
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
                    while stack.len()>0 && stack.last().unwrap().to_string() != String::from("(") && get_priority(elt) < get_priority(stack.last().unwrap()){
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




        fn evaluate_postfix_exp(&self, exp: &str) -> bool{
            let mut stack = Vec::<&str>::new();
            let split_exp: Vec<&str> = exp.split_whitespace().collect();
            for elt in split_exp{
                if self.operator_list.contains(&elt){
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    if compare_to_valid_element(left, elt, right){
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

    }

    pub fn split(string: &str, splitter: &str) -> Vec::<String>{
        string.split(splitter).map(String::from).collect()
    }

    pub fn count_occur(string: &str, x: char) -> i32{
        let mut count = 0;
        for chara in string.chars(){
            if chara == x {
                count += 1;
            }
        }
        return count;
    }

    pub fn convert_bool_to_number(string: &str) -> String{
        match string{
            "true" => return String::from("1"),
            _ => return String::from("0")
        }
    }

    pub fn hash_string_to_number(string: String)->i32{
        let mut result: i32 = 0;
        for chara in string.chars(){
            result += chara as i32;
        }
        result
    }


    fn compare_to_valid_element(left_s: &str, operator: &str, right_s: &str) -> bool{
        let left: f32 = String::from(left_s).parse().unwrap_or_default();
        let right: f32 = String::from(right_s).parse().unwrap_or_default();
        match operator{
            "==" => return left == right,
            "!=" => return left != right,
            ">" => return left > right,
            "<" => return left < right,
            ">=" => return left >= right,
            "<=" => return left <= right,
            "AND" => return left == 1.0 && right == 1.0,
            "OR" => return left == 1.0 || right == 1.0,
            _ => return false
        }
    }

    fn get_priority(operator: &str) -> i32{
        if operator == "AND" || operator == "OR"{
            return 1;
        }else if operator == "("{
            return 3;
        }else{
            return 2;
        }
    }

    pub fn good_type_and_good_value(type_value: &str, value: &str) -> bool{
        match type_value{
            "BOOL" => return value == "false" || value == "true",
            "STRING" => return true,
            "INT" => return is_int(value),
            _ => return is_float(value)
        }
    }

    pub fn is_int(string : &str) -> bool{
        let numbers = "1234567890";
        for chara in string.chars(){
            if !numbers.contains(chara.clone()){
                return false;
            } 
        }
        true
    }

    pub fn is_float(string : &str) -> bool{
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

    pub fn is_value(string: &str) -> bool{
        return is_float(string);
    }
}