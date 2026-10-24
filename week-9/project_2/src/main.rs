use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    let mut file = File::create("pau_students.txt")
        .expect("Failed to create file");

    for student in &students {
        println!("Student Name: {}", student.0);
        println!("Matric Number: {}", student.1);
        println!("Department: {}", student.2);
        println!("Level: {}", student.3);
        println!("-------------------------");

        let details = format!(
            "Student Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n\n",
            student.0, student.1, student.2, student.3
        );

        file.write_all(details.as_bytes())
            .expect("Failed to write as file");
    }

    println!("Your student records have been saved successfully, have a nice day!");
}