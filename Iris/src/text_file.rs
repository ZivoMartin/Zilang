use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::io::BufRead;
use std::io::Seek;
use std::process;

pub static SRC_PATH: &str = "/net/cremi/mzivojinovic/Travail/Vulcain/Iris/";

pub struct TextFile{
    file_path: PathBuf,
    file: File
}

pub fn get_file(file_path: String) -> File {
    if !file_exists_brut(&file_path){
        create_file(&file_path);
    }
    match fs::OpenOptions::new().append(true).read(true).open(&file_path){
        Ok(f) => f,
        Err(e) => {
            println!("Erreur lors de l'ouverture du fichier {}: {}", file_path, e);
            process::exit(0);
        }
    }
}

impl TextFile{

    pub fn new(file_path: String) -> TextFile {
        TextFile {
            file_path: PathBuf::from(src_path() + &file_path),
            file: get_file(src_path() + &file_path)
        }
    }

    pub fn new_brut(file_path: String) -> TextFile {
        TextFile {
            file_path: PathBuf::from(&file_path),
            file: get_file(file_path)
        }
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
            println!("{:?} Le fichier n'a pas été supprimé: {}", self.file_path, e);
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

pub fn src_path() -> String {
    SRC_PATH.to_string()
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(src_path() + file_path).is_ok()
}

pub fn file_exists_brut(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

fn create_file(file_path: &str){
    let _ = File::create(file_path).map_err(|e|{
        println!("Erreur lors de la creation du fichier {}: {}", file_path, e);
    });
}