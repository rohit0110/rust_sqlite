pub struct Settings {
    id: usize,
    done: bool,
    description: String,
}

impl Settings {
    pub fn new(id: usize, description: String) -> Settings{
        Settings {
            id,
            description,
            done: false
        }
    }

    pub fn print_details(&self) {
        println!("Description of the task is {}", self.description);
        println!("The task is {}", self.done);
    }
}