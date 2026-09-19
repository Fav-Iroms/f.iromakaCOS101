use std::io;

fn main() {
    println!("Are you experienced? (yes/no)");

    let mut experience = String::new();
    io::stdin().read_line(&mut experience).unwrap();

    let experience = experience.trim();

    println!("Enter your age:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let age: i32 = input.trim().parse().unwrap();

    if experience == "yes" {
        if age >= 40 {
            println!("Annual incentive is ₦1,560,000");
        } else if age >= 30 {
            println!("Annual incentive is ₦1,480,000");
        } else {
            println!("Annual incentive is ₦1,300,000");
        }
    } else {
        println!("Annual incentive is ₦100,000");
    }
}