fn display(v: Vec<i32>) -> Vec<i32> {
    println!("Vector: {:?}", v);

    v
}

fn main() {
    let v = vec![101, 250, 330, 400];

    let v2 = display(v);

    println!("Returned vector: {:?}", v2);
}