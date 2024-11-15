pub struct Settings {
    pub id: i32,
    pub done: bool,
    pub description: String,
}

impl Settings {
    pub fn new(id: i32, description: String) -> Settings{
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