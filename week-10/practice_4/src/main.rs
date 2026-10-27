fn print_vector(v: Vec<i32>) {
    println!("Vector: {:?}", v);
}

fn main() {
    let v = vec![101, 250, 330, 400];

    print_vector(v.clone());

    println!("First element: {}", v[0]);
}