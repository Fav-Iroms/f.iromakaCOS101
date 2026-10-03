use std::io;

fn main() {
    println!("Menu:");
    println!("P - Poundo Yam / Edinkaiko Soup (N3,200)");
    println!("F - Fried Rice & Chicken (N3,000)");
    println!("A - Amala & Ewedu Soup (N2,500)");
    println!("E - Eba & Egusi Soup (N2,000)");
    println!("W - White Rice & Stew (N2,500)");

    println!("\nEnter food type (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).unwrap();
    let food_type = food_type.trim().to_uppercase();

    let price = if food_type == "P" {
        3200.0
    } else if food_type == "F" {
        3000.0
    } else if food_type == "A" {
        2500.0
    } else if food_type == "E" {
        2000.0
    } else if food_type == "W" {
        2500.0
    } else {
        println!("Invalid selection!");
        return;
    };

    println!("Enter quantity:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).unwrap();
    let quantity: f64 = quantity_input.trim().parse().unwrap();

    let mut total = price * quantity;

    if total > 10000.0 {
        total = total - (total * 0.05);
    }

    println!("Total Charge: N{}", total);
}