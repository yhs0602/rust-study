#![feature(slice_pattern)]
extern crate core;

use core::slice::SlicePattern;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the company management program!");
    println!("Type 'Add Sally to Engineering' to add one to a department");
    println!("Type 'List Engineering' to list people in the department");
    println!("Type 'ListAlpha Engineering' to list people in the departmentin alphabet order");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        // read one line
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        // split command
        let tokens: Vec<&str> = line.trim().split_whitespace().into_iter().collect();
        match tokens.as_slice() {
            [command, rest @ ..] => {
                match command.to_lowercase().as_str() {
                    "add" => {
                        println!("Adding");
                        // sally, to, enginnering
                        match rest.as_slice() {
                            [who, "to", department] => {
                                let iter = departments.entry(String::from(*department)).or_insert_with(Vec::new);
                                iter.push(String::from(*who));
                            }
                            _ => {
                                println!("Unexpected input");
                            }
                        }
                    }
                    "list" => {
                        println!("Listing");
                        match rest.as_slice() {
                            [department] => {
                                // iterate over the list
                                if let Some(members) = departments.get(&String::from(*department)) {
                                    for member in members {
                                        println!("Member {}", member);
                                    }
                                } else {
                                    println!("No members in {}", department);
                                }
                            }
                            _ => {
                                println!("Unexpected input");
                            }
                        }
                    }
                    "listalpha" => {
                        println!("Listing alphabetically");
                        match rest.as_slice() {
                            [department] => {
                                // iterate over the list
                                if let Some(members) = departments.get(&String::from(*department)) {
                                    // sort alphabetically and print
                                    let mut cloned = members.clone();
                                    cloned.sort();
                                    for member in cloned {
                                        println!("Member {}", member);
                                    }
                                } else {
                                    println!("No members in {}", department);
                                }
                            }
                            _ => {
                                println!("Unexpected input");
                            }
                        }
                    }
                    _ => {
                        println!("Unexpected input");
                    }
                }
            }
            _ => {
                println!("Unexpected input");
            }
        }
    }
}
