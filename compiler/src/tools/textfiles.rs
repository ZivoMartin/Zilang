use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::io::BufRead;
use std::io::Seek;
use std::fs;
use std::fs::File;

static SRC_PATH: &str = "/home/martin/Travail/Vulcain/compiler/";

#[allow(dead_code)]
pub struct TextFile{
    file_path: PathBuf,
    file: File,
}

#[allow(dead_code)]
impl TextFile{

    pub fn new(mut file_path: String) -> Result<TextFile, String> {
        file_path = SRC_PATH.to_string() + &file_path;
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
    
    #[allow(dead_code)]
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
}

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

fn create_file(file_path: &str){
    let _ = File::create(&file_path).map_err(|e|{
        println!("Erreur lors de la creation du fichier {}: {}", file_path, e);
    });
}