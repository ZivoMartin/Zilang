mod system;
mod text_file;
mod interpreteur;
mod type_gestion;
mod view;

use crate::view::View;

fn main() -> Result<(), String>{
    let mut view = View::new()?;
    view.start()
}

