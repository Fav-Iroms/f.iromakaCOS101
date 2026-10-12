use std::io;

fn main() {
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Enter job title (e.g. Associate):");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");
    let title = title.trim();

    println!("Enter claimed level (e.g. APS 5-8):");
    let mut claimed = String::new();
    io::stdin().read_line(&mut claimed).expect("Failed to read input");
    let claimed = claimed.trim();

    let mut found = false;

    for i in 0..levels.len() {
        if title == office_admin[i] || title == academic[i] || title == lawyer[i] || title == teacher[i] {
            found = true;
            if claimed == levels[i] {
                println!("VALID: {} is {}", title, levels[i]);
            } else {
                println!("INVALID: {} should be {}", title, levels[i]);
            }
        }
    }

    if !found {
        println!("Job title not found.");
    }
}