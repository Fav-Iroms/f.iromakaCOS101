use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("\n=== The Shape Calculator ===");
        println!("1. Trapezium Area");
        println!("2. Rhombus Area");
        println!("3. Parallelogram Area");
        println!("4. Cube Surface Area");
        println!("5. Cylinder Volume");
        println!("6. Exit");
        print!("Choose an option (1-6): ");
        
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => println!("Result: {:.2}\n", calculate_trapezium()),
            2 => println!("Result: {:.2}\n", calculate_rhombus()),
            3 => println!("Result: {:.2}\n", calculate_parallelogram()),
            4 => println!("Result: {:.2}\n", calculate_cube()),
            5 => println!("Result: {:.2}\n", calculate_cylinder()),
            6 => break,
            _ => println!("Invalid choice.\n"),
        }
    }
}

fn read_input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => print!("Invalid number. "),
        }
    }
}

fn calculate_trapezium() -> f64 {
    let height = read_input("Height: ");
    let base1 = read_input("Base 1: ");
    let base2 = read_input("Base 2: ");
    height / 2.0 * (base1 + base2)
}

fn calculate_rhombus() -> f64 {
    let diagonal1 = read_input("Diagonal 1: ");
    let diagonal2 = read_input("Diagonal 2: ");
    0.5 * diagonal1 * diagonal2
}

fn calculate_parallelogram() -> f64 {
    let base = read_input("Base: ");
    let altitude = read_input("Altitude: ");
    base * altitude
}

fn calculate_cube() -> f64 {
    let side = read_input("Side: ");
    6.0 * side * side
}

fn calculate_cylinder() -> f64 {
    let radius = read_input("Radius: ");
    let height = read_input("Height: ");
    std::f64::consts::PI * radius * radius * height
}