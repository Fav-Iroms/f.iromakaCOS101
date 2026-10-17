fn main() {
    let developers = vec![
        ("Grace", 4),
        ("Effiong", 8),
        ("Basil", 2),
        ("Kareem", 10),
        ("Susan", 6),
    ];

    let mut best_developer = developers[0].0;
    let mut max_years = developers[0].1;

    for i in 0..developers.len() {
        let current_name = developers[i].0;
        let current_years = developers[i].1;

        if current_years > max_years {
            max_years = current_years;
            best_developer = current_name;
        }
    }

    print!("{} has the highest years of programming experience with {} years\n", best_developer, max_years);
}