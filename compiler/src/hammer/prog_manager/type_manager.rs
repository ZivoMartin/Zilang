use super::prog_manager::ProgManager;


impl ProgManager {

    pub fn get_type_size(&self, nb_s: i32, name: &str) -> u8 {
        if nb_s != 0 {
            4
        }else{
            self.get_type_size_with_type_name(name)
        }
    }

    pub fn get_type_name_with_id(self, id: usize) -> String {
        self.titn[id].clone()
    }

    pub fn get_type_size_with_id(&self, id: usize) -> u8 {
        self.tnti.get(&self.titn[id]).expect(&format!("Unvalid id: {id}")).0
    }

    pub fn get_type_size_with_type_name(&self, type_name: &str) -> u8 {
        self.tnti.get(type_name).expect(&format!("Type {type_name} doesn't exists")).0
    }

    pub fn get_type_id_with_type_name(&self, type_name: &str) -> usize {
        self.tnti.get(type_name).expect(&format!("Type {type_name} doesn't exists")).1
    }

}