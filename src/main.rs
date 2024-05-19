mod db_controller;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::db_controller::Database;

fn main() -> rusqlite::Result<()> {
    let db = Database::new("my_database.db")?;
    db.initialise_database()?;

    menu_sel(&db);
    Ok(())
}

fn menu_sel(db: &Database) {
    loop {
        println!("1. Add new entry");
        println!("2. Show all entries");
        println!("3. Remove entry");
        io::stdout().flush().unwrap();
        let choice = get_int_input_loop();
        match choice {
            1 => add_new_entry(&db),
            2 => print_all_entries(&db),
            3 => remove_entry(&db),
            _ => println!("Wrong choice!")
        }
    }
}

fn add_new_entry(db: &Database) {
    let mut new_key = String::new();
    fill_str_input_loop(&mut new_key, "Enter the new key: ");
    let new_value = get_int_input_loop();
    match db.add_entry_to_db(&new_key, new_value) {
        Ok(()) => println!("New Entry Created!"),
        Err(err) => println!("Error: {}", err),
    }
}

fn print_all_entries(db: &Database) {
    let mut map: HashMap<String, i32> = HashMap::new();
    match db.load_hashmap_from_db(&mut map) {
        Ok(()) => {
            println!("Hash Map's contents are:");
            for (key, value) in map {
                println!("{}: {}", key, value);
            }
        },
        Err(error) => println!("Error Encountered: {}", error),
    };
}

fn fill_str_input_loop(parse_str: &mut String, input_msg: &str) {
    loop {
        if let Some(input) = get_str_input(&input_msg) {
            parse_str.push_str(&input);
            break;
        }
    }
}

fn get_str_input(input_msg: &str) -> Option<String> {
    let mut input = String::new();

    print!("{}", input_msg);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = input.trim();

    if !trimmed.is_empty() {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn remove_entry(db: &Database) {
    print_all_entries(&db);
    let mut key = String::new();
    fill_str_input_loop(&mut key, "Enter key to delete: ");

    match db.remove_entry_by_key(&key) {
        Ok(()) => println!("Deletion Successful!"),
        Err(err) => println!("Deletion Failed: {}", err),
    }
}


fn get_int_input_loop() -> i32 {
    let mut input = None;

    while input.is_none() {
        input = get_int_input("Enter a number: ");
    }

    input.unwrap()
}

fn get_int_input(input_msg: &str) -> Option<i32> {
    let mut input = String::new();

    print!("{}", input_msg);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}
