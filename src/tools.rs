
#[allow(dead_code)]
pub mod tools{
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Write;
    use std::path::PathBuf;
    use std::io::BufRead;
    use std::io::Seek;
    use crate::stack::Stack;
    use std::collections::HashMap;

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
        authorized_char_for_variable: &'static str,
        operators: &'static str,
        operator_priority: HashMap<&'static str, u8>
    }

    impl Tools{

        pub fn new() -> Tools{
            Tools{
                authorized_char_for_variable: "azertyuiopqsdfghjklmwxcvbnAZERTYUIOPQSDFGHJKLMWXCVBN1234567890-_",
                operators: "+-*/",
                operator_priority: build_operator_priority()
            }
        }

        pub fn is_valid_name(&self, name: &str) -> bool{
            for letter in name.chars(){
                if !self.authorized_char_for_variable.contains(letter){
                    return false;
                }
            }
            true
        }

        pub fn is_operator(&self, x: String) -> bool{
            self.operators.contains(&x)
        }

        pub fn convert_in_postfix_exp(&self, exp: Vec::<String>) -> Vec::<String>{
            let mut result = Vec::<String>::new();
            let mut stack = Stack::<String>::new();

            for elt in exp.iter(){
                if self.is_operator(String::from(elt)){
                    while !stack.is_empty() && *stack.val() != String::from("(") && self.get_priority(elt) < self.get_priority(stack.val()){
                        result.push(stack.pop());
                    }
                    stack.push(String::from(elt));
                }else if elt == ")"{
                    while stack.val() != "(" {
                        result.push(stack.pop());
                    }
                    stack.pop();
                }else{
                    result.push(String::from(elt));
                }
            }
            while stack.size() != 0 {
                result.push(stack.pop());
            }
            result
        }

        fn get_priority(&self, operator: &str) -> u8{
            if operator == "+" || operator == "-"{
                return 1;
            }else if operator == "("{
                return 3;
            }else{
                return 2;
            }
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

    fn build_operator_priority() -> HashMap<&'static str, u8>{
        let mut res = HashMap::<&'static str, u8>::new();
        res.insert("+", 1);
        res.insert("-", 1);
        res.insert("*", 2);
        res.insert("/", 2);
        res.insert("(", 3);
        res.insert(")", 2);
        res
    }
  

}