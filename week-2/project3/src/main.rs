fn main() {
    // Given parameters
    let p: f64 = 210_000.0; // Principal value in Naira
    let r: f64 = 5.0;       // Depreciation rate (5%)
    let n: f64 = 3.0;       // Time in years

    // Formula: A = P * (1 - R/100)^n
    let a: f64 = p * (1.0 - (r / 100.0)).powf(n);

    // Display output
    println!("--- Project III: Depreciation Calculation ---");
    println!("Original TV Value (P): N{:.2}", p);
    println!("Depreciation Rate (R): {}%", r);
    println!("Time Period (n): {} years", n);
    println!("Value of TV after 3 years (A): N{:.2}", a);
}
