struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn display(emp: Employee) {
    println!("Name: {}", emp.name);
    println!("Company: {}", emp.company);
    println!("Age: {}", emp.age);
    println!();
}

fn main() {
    let emp1 = Employee {
        name: String::from("Satya Nadella"),
        company: String::from("Microsoft"),
        age: 58,
    };

    let emp2 = Employee {
        name: String::from("Sundar Pichai"),
        company: String::from("Google"),
        age: 54,
    };

    display(emp1);
    display(emp2);
}