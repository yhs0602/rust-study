use std::io;

fn main() {
    println!("Welcome to the company management program!");
    println!("Type 'Add Sally to Engineering' to add one to a department");
    println!("Type 'List Engineering' to list people in the department");
    println!("Type 'ListAlpha Engineering' to list people in the departmentin alphabet order");
    loop {
        // read one line
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        // split command
        let tokens: Vec<String> = line.trim().split_whitespace().collect();
        match tokens {
            [command, rest @ ..] => {
                match *command.to_lowercase() {
                    "add" => {
                        println!("Adding");
                    }
                    "list" => {
                        println!("Listing");
                    }
                    "listalpha" => {
                        println!("Listing alphabetically");
                    }
                }
            }
        }
    }
}
