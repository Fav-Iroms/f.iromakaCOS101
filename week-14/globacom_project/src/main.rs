
use std::fs;
use std::io::{self, Write};

fn read_file(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("\n{}", contents);
        }
        Err(error) => {
            println!("Error reading file: {}", error);
        }
    }
}

fn administrator() {
    println!("\nADMINISTRATOR");
    read_file("globacom_db.sql");
}

fn project_manager() {
    println!("\nPROJECT MANAGER");
    read_file("project_tb.sql");
}

fn employee() {
    println!("\nEMPLOYEE");
    read_file("staff_tb.sql");
}

fn customer() {
    println!("\nCUSTOMER");
    read_file("customer_tb.sql");
}

fn vendor() {
    println!("\nVENDOR");
    read_file("dataplan_tb.sql");
}

fn main() {
    println!("================================");
    println!("       GLOBACOM DATABASE");
    println!("================================");

    println!("Select your role:");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => administrator(),
        "2" => project_manager(),
        "3" => employee(),
        "4" => customer(),
        "5" => vendor(),
        _ => println!("Invalid choice!"),
    }
}
