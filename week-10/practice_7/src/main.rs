struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn main() {
    let emp1 = Employee {
        name: String::from("John"),
        company: String::from("Microsoft"),
        age: 30,
    };

    println!("Name: {}", emp1.name);
    println!("Company: {}", emp1.company);
    println!("Age: {}", emp1.age);
}