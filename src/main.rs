use std::env;

#[derive(Debug)]
struct Thing {
    id: u32,
    definition: String,
    note: String,
    focus: String,
    completed: bool    
}

impl Thing {
    fn new(id: u32, definition: &str, note: &str, focus: &str) -> Self {
        Thing{
            id,
            definition: String::from(definition),
            note: String::from(note),
            focus: String::from(focus),
            completed: false,
        }
    }

    fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}
struct Things {
    items: Vec<Thing>,
}

impl Things {
    fn new() -> Self {
        Things { items: Vec::new() }
    }
    
    fn add(&mut self, thing: Thing) {
        self.items.push(thing);
    }

    fn remove(&mut self, id: u32) -> Option<Thing> {
        if let Some(pos) = self.items.iter().position(|x| x.id == id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
   }

   fn list(&self) {
        for thing in &self.items {
            println!("Thing - {} | {}", thing.definition, thing.focus);
            if !thing.note.is_empty() {
                println!("{}", thing.note);
            }
        }
   }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
