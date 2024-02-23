use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color; 
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::Sdl;  
use std::time::Duration;
use std::path::Path;
use sdl2::ttf::Font;

use crate::interpreteur::{Interpreteur, ResponseData};
use crate::text_file::TextFile;
use crate::text_file::file_exists;

pub struct View{
    context: Sdl,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    interpreteur: Interpreteur,
    cursor_pos: Xy,
    case_size: Xy,
    background_color: Color,
    iter: u32,
    char_tab: Vec<String>,
    ctrle: bool
}

struct Xy{
    x: u32,
    y: u32
}

impl Xy {

    fn new(x: u32, y: u32) -> Xy{
        Xy{x, y}
    }
    
    fn change(&mut self, x: u32, y: u32){
        self.x = x;
        self.y = y;
    }
}


impl View{

    pub fn new(interpreteur: Interpreteur) -> Result<View, String> {
        let size_window = Xy::new(1400, 800);
        let case_size = Xy::new(15, 40);
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Iris", size_window.x, size_window.y)
            .position_centered()
            .build()
            .expect("could not initialize video subsystem");
        
        let canvas = window.into_canvas().build().expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        let mut char_vec: Vec<String> = Vec::new();
        let height = size_window.y/case_size.y;
        char_vec.push(String::from(">"));
        for _ in 1..height{
            char_vec.push(String::from(" "));
        }
        Ok(View{
            context: sdl_context,
            canvas,
            texture_creator,
            interpreteur,
            cursor_pos: Xy::new(1, 0),
            case_size,
            background_color: Color::RGB(0, 0, 0),
            iter: 0,
            char_tab: char_vec,
            ctrle: false
        })
    }

    fn action(&mut self, font: &Font) -> Result<(), String> {
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..self.char_tab.len(){
            let txt = self.char_tab[i].clone();
            self.draw_text(i as i32, &txt, &font)?;
        }
        if !(self.iter % 60 <= 10){    
            self.draw_cursor();
        }
        self.canvas.present();
        Ok(())
    }


    fn draw_cursor(&mut self){
        self.canvas.fill_rect(Rect::new((self.cursor_pos.x*self.case_size.x) as i32, (self.cursor_pos.y*self.case_size.y) as i32, self.case_size.x, self.case_size.y)).expect("Failed to draw rectangle");
    }

    fn draw_text(&mut self, y: i32, txt: &str, font: &Font) -> Result<(), String>{
        let surface = font.render(txt).blended(Color::RGBA(255, 255, 255, 0)).map_err(|e| e.to_string())?;
        let texture = self.texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let target = Rect::new(0, y*(self.case_size.y as i32), self.case_size.x*(txt.len() as u32), self.case_size.y);
        self.canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {
        let mut event_pump = self.context.event_pump()?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
        let font = ttf_context.load_font(Path::new(&"fonts/OpenSans-Bold.ttf"), 128)?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    Event::KeyDown { keycode, ..} => {
                        match keycode.unwrap(){
                            Keycode::LCtrl | Keycode::RCtrl => self.ctrle = true,
                            Keycode::Backspace => self.delete_char(),
                            Keycode::V => {
                                if self.ctrle{
                                    
                                }
                            },
                            Keycode::Left => {
                                if self.cursor_pos.x > 1{
                                    self.cursor_pos.x -= 1;
                                }
                            },
                            Keycode::Right => {
                                if self.cursor_pos.x < self.char_tab[self.cursor_pos.y as usize].len() as u32{
                                    self.cursor_pos.x += 1;
                                }
                            },
                            Keycode::Return => self.entry_key(),
                            Keycode::Escape => break 'running,
                            _ => {}
                        }
                    },
                    Event::KeyUp { keycode, ..} => {
                        match keycode.unwrap(){
                            Keycode::LCtrl | Keycode::RCtrl => self.ctrle = false,
                            _ => {}
                        }
                    },
                    Event::TextInput { text, .. } => {
                        if !text.is_empty() {
                            self.new_entry(&text);
                        }
                    }
                    _ => {} 
                }
            }
    
            self.action(&font)?;
            self.iter += 1;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    fn delete_char(&mut self){
        if self.cursor_pos.x > 1 && self.char_tab[self.cursor_pos.y as usize].len()>1{
            self.cursor_pos.x -= 1;
            self.char_tab[self.cursor_pos.y as usize].remove(self.cursor_pos.x as usize);
        }
    }

    fn new_entry(&mut self, text: &str){
        self.char_tab[self.cursor_pos.y as usize].insert_str(self.cursor_pos.x as usize, text);
        self.cursor_pos.x += 1;
    }

    fn entry_key(&mut self){
        self.char_tab[self.cursor_pos.y as usize].remove(0);
        let text = &self.char_tab[self.cursor_pos.y as usize];
        if text == "clear"{
            for i in 0..(self.cursor_pos.y+1){
                self.char_tab[i as usize] = String::from(" ");
            }
            self.char_tab[0] = String::from(">");
            self.cursor_pos.change(1, 0);
        }else if text.starts_with(r"\i"){
            let split: Vec<&str> = text.split_whitespace().collect(); 
            if split.len() == 2{
                if file_exists(split[1]){
                    if split[1].ends_with(".sql") || split[1].ends_with(".txt"){
                        let mut sql_file = TextFile::new(split[1].to_string());
                        let mut f_text = sql_file.get_text();
                        f_text = f_text.replace("\n", "");
                        let mut all_request: Vec<&str> = f_text.split(";").collect();
                        all_request.pop();
                        for req in all_request{
                            self.new_request(req.to_string());
                        }
                    }else{
                        self.error_message(r"The file in arguments of \i have to be a sql file.");
                    }
                }else{
                    self.error_message(&format!("The path {} is not valid.", split[1]));
                }
            }else{
                self.error_message(r"\i take only a path to a .sql file in argument. Two arguments found here.");
            }
            self.replace_cursor();
        }else{
            self.new_request(text.to_string());
            self.replace_cursor();
        }
    }

    fn new_request(&mut self, text: String){
        match self.interpreteur.sqlrequest(text.to_string(), ResponseData::new_empty()){
            Ok(res) => {
                match res{
                    Some(result) => {
                        let keys: Vec::<String> = result.keys().cloned().collect();
                        if keys.len() > 0{
                            let mut i: usize = (self.cursor_pos.y + 1) as usize;
                            self.cursor_pos.y += (result[&keys[0]].len() + 1) as u32;
                            for key in &keys{
                                self.char_tab[i].push_str(&format!("| {} |", key));
                            }
                            for k in 0..result[&keys[0]].len(){
                                i += 1;
                                for j in 0..keys.len(){
                                    self.char_tab[i].push_str(&format!("|{}|", result[&keys[j]][k]));
                                }
                            }
                        }
                    }
                    None => {}
                }
            }   
            Err(e) => {
               self.error_message(&e);
            }
        }
    }

    fn error_message(&mut self, e: &str){
        if self.cursor_pos.y < (self.char_tab.len()-1) as u32{
            self.cursor_pos.y += 1;
        }
        self.char_tab[self.cursor_pos.y as usize] = e.to_string();
    }

    fn replace_cursor(&mut self){
        self.cursor_pos.change(1, self.cursor_pos.y + 1);
        if self.cursor_pos.y >= self.char_tab.len() as u32{
            let nb_line_to_delete = self.cursor_pos.y - (self.char_tab.len() as u32) + 1;
            self.cursor_pos.y = (self.char_tab.len() - 1) as u32;
            for _ in 0..nb_line_to_delete{
                for i in 0..(self.char_tab.len()-1){
                    self.char_tab[i] = self.char_tab[i+1].clone()
                }
            }
            self.cursor_pos.y = (self.char_tab.len()-1) as u32;
        }
        self.char_tab[(self.cursor_pos.y) as usize] = String::from(">");
        if self.char_tab[(self.cursor_pos.y-1) as usize].len() == 0{
            self.char_tab[(self.cursor_pos.y-1) as usize] = String::from(" ");
        }
    }
}
