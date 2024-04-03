use super::{include::*, prog_manager::ProgManager};


impl ProgManager {

    pub fn get_type_size(&self, nb_s: i32, name: &str) -> u8 {
        if nb_s != 0 {
            4
        }else{
            self.get_type_size_with_type_name(name)
        }
    }

    pub fn get_type_name_with_id(&self, id: usize) -> String {
        self.titn[id].clone()
    }

    pub fn _get_type_size_with_id(&self, id: usize) -> u8 {
        self.tnti.get(&self.titn[id]).expect(&format!("Unvalid id: {id}")).0
    }

    pub fn get_type_size_with_type_name(&self, type_name: &str) -> u8 {
        self.tnti.get(type_name).expect(&format!("Type {type_name} doesn't exists")).0
    }

    pub fn get_type_id_with_type_name(&self, type_name: &str) -> usize {
        self.tnti.get(type_name).expect(&format!("Type {type_name} doesn't exists")).1
    }

    pub fn _nb_base_type(&self) -> usize {
        TYPE_LIST.len()
    }

    /// Add a class in the memory and return his id
    pub fn add_class(&mut self, new_class_name: String) {
        self.current_class = new_class_name.clone();
        let new_class = Class::new(new_class_name, self.titn.len());
        self.tnti.insert(new_class.get_name().clone(), (POINTER_SIZE as u8, self.titn.len()));
        self.titn.push(new_class.get_name().clone());
        self.class_name_map.insert(new_class.get_name().clone(), new_class);
    }

    pub fn class_exists(&self, name: &str) -> bool {
        self.class_name_map.contains_key(name)
    }

    pub fn get_class_by_name(&self, name: &String) -> &Class {
        self.class_name_map.get(name).expect(&format!("The class {} doesn't exists", name))
    }

    pub fn get_class(&self, id: usize) -> &Class {
        self.get_class_by_name(&self.get_type_name_with_id(id))
    }

    pub fn get_class_by_name_mut(&mut self, name: &String) -> &mut Class {
        self.class_name_map.get_mut(name).expect(&format!("The class {} doesn't exists", name))
    }

    pub fn get_class_mut(&mut self, id: usize) -> &mut Class {
        self.get_class_by_name_mut(&self.get_type_name_with_id(id))
    }
}