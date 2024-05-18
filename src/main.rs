mod db_controller;

use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    initialise_hashmap(&mut hashmap);

    menu_sel(&mut hashmap);
}

fn menu_sel(map: &mut HashMap<String, i32>) {
    loop {
        println!("1. Add new entry");
        println!("2. Show all entries");
        io::stdout().flush().unwrap();
        let choice = get_int_input_loop();
        match choice {
            1 => add_new_to_hashmap(map),
            2 => print_hashmap(map),
            _ => println!("Wrong choice!")
        }
    }
}

fn initialise_hashmap(map: &mut HashMap<String, i32>) {
    map.insert("apple".to_string(), 10);
    map.insert("banana".to_string(), 20);
    map.insert("orange".to_string(), 15);
}

fn add_new_to_hashmap(map: &mut HashMap<String, i32>) {
    let mut new_key = String::new();
    fill_str_input_loop(&mut new_key);
    let new_value = get_int_input_loop();
    map.entry(new_key)
        .and_modify(|v| *v = new_value)
        .or_insert(new_value);
}

fn print_hashmap(map: &mut HashMap<String, i32>) {
    println!("Hash Map's contents are:");
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

fn fill_str_input_loop(parse_str: &mut String) {
    loop {
        if let Some(input) = get_str_input() {
            parse_str.push_str(&input);
            break;
        }
    }
}

fn get_str_input() -> Option<String> {
    let mut input = String::new();

    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = input.trim();

    if !trimmed.is_empty() {
        Some(trimmed.to_string())
    } else {
        None
    }
}


fn get_int_input_loop() -> i32 {
    let mut input = None;

    while input.is_none() {
        input = get_int_input();
    }

    input.unwrap()
}

fn get_int_input() -> Option<i32> {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
