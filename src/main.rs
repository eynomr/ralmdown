use std::env;
use std::fs::read_link;
use std::io::{self, Write};

#[derive(Debug)]
struct Thing {
    definition: String,
    note: String,
    focus: String,
    completed: bool    
}

impl Thing {
    fn new(definition: &str, note: &str, focus: &str) -> Self {
        Thing{
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

    fn remove(&mut self, pos: usize) -> Option<Thing> {
        if pos <= self.items.len() {
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

fn interactive_mode(things: &mut Things) {
    println!("ralmdown. Type 'help' for a list of commands.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "help" => {
                println!("ralmdown commands:");
                println!("    add <definition> <note> <focus> - add a new thing");
                println!("    list - list all the things");
                println!("    exit - exit the interactive mode");
            }
            "list" => {
                things.list();
            }
            "add" => {
                print!("definition: ");
                io::stdout().flush().unwrap();
                let mut definition = String::new();
                io::stdin().read_line(&mut definition).unwrap();
                let definition =  definition.trim();               
                
                print!("note: ");
                io::stdout().flush().unwrap();
                let mut note = String::new();
                io::stdin().read_line(&mut note).unwrap();
                let note =  note.trim();

                print!("focus: ");
                io::stdout().flush().unwrap();
                let mut focus = String::new();
                io::stdin().read_line(&mut focus).unwrap();
                let focus =  focus.trim();

                let thing = Thing::new(definition, note, focus);
                println!("Thing added.");
                println!("{:?}", thing);
                things.add(thing);

            }
            cmd if cmd.starts_with("add") => {
                
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.len() < 4 {
                    println!("Usage: add <definition> <note> <focus>");
                    continue;
                }
                let thing = Thing::new(parts[1], parts[2], parts[3]);
                println!("Thing added.");
                println!("{:?}", thing);
                things.add(thing);
            }
            "exit" => {
                println!("bye");
                break;
            }
            _ => {
                println!("Unknown command. 'help' for list of command.");
            }
        }
    }
    
}
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let mut things = Things::new();
    // what do we expect the commands to be? 

    if args.len() == 2 && args[1] == "interactive" {
        interactive_mode(&mut things);
    }

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [options]", args[0]);
        return;
    }

    if args.len() > 5 {
        eprintln!("Too many commands\nUsage: {} <command> [options]", args[0]);
        return;
    }

    match args[1].as_str() {
        "add" => {
            let definition = args[2].as_str();
            if args.len() == 3 {
                let thing = Thing::new(definition, "", "ali");
                things.add(thing);
            } else if args.len() == 5 {
                let note = args[3].as_str();
                let focus = args[4].as_str();
                let thing = Thing::new(definition, note, focus);
                things.add(thing);
            }

            println!("Thing added.");
            things.list();
        }
        _ => {
            eprint!("Unkown command: {}", args[1]);
        }
    }

}
