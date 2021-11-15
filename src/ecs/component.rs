#[derive(Debug)]
pub struct health(pub i32);
#[derive(Debug)]
pub struct name(pub &'static str);

#[derive(Debug)]
pub struct world {
    healt_components: Vec<Option<health>>,
    name_components: Vec<Option<name>>,
}

impl world {
    pub fn new() -> Self {
        Self {
            healt_components: Vec::new(),
            name_components: Vec::new(),
        }
    }

    pub fn new_entity(&mut self, heel: Option<health>, namee: Option<name>) {
        self.healt_components.push(heel);
        self.name_components.push(namee);
    }
}







