fn main(){
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let years: i32 = 5;

    let amount = principal * (1.0 + rate / 100.0) .powi(years);

    let compound_interest = amount - principal;
    println!("Principal: ₦{:.2}", principal);
    println!("Rate: {}%", rate);
    println!("Time: {} years", years);
    println!("Amount: ₦{:.2}", amount);
    println!("Compound Interest: ₦{:.2}", compound_interest);
}