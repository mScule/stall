pub struct ModIndex {
    index: Vec<String>,
}

impl ModIndex {
    pub fn new() -> Self {
        Self { index: Vec::new() }
    }
    pub fn get_index_of_mod(&mut self, mod_name: &String) -> usize {
        match self.index.iter().position(|name| name.eq(mod_name)) {
            Some(mod_index) => mod_index,
            None => {
                self.index.push(mod_name.clone());

                self.index.len() - 1
            }
        }
    }
}
