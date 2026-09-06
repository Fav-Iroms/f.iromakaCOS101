fn main() {
    // Sales amounts for Toshiba, Mac, HP, Dell, and Acer
    let amounts: [f64; 5] = [
        450_000.00,   // Toshiba
        1_500_000.00, // Mac
        750_000.00,   // HP
        2_850_000.00, // Dell
        250_000.00,   // Acer
    ];

    // Calculate total sum
    let sum: f64 = amounts.iter().sum();

    // Calculate average
    let average: f64 = sum / (amounts.len() as f64);

    // Display results
    println!("--- P.M. Okeke and Sons Ltd Sales Report ---");
    println!("Total Sum of Sales: N{:.2}", sum);
    println!("Average Sales Amount: N{:.2}", average);
}