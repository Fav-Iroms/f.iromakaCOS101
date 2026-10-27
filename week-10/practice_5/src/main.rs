fn borrow_vector(z: &Vec<i32>) {
    println!("Vector: {:?}", z);
}

fn main() {
    let x = vec![101, 250, 330, 400];

    borrow_vector(&x);

    println!("First element: {}", x[0]);
}