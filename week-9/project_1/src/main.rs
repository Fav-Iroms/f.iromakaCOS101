
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("nigerian_breweries.txt")
        .expect("Failed to create file");

    file.write_all(b"LAGER\n") .expect("Write failed");
    file.write_all(b"33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar\n\n").expect("Write failed");

    file.write_all(b"STOUT\n")
        .expect("Write failed");
    file.write_all(b"Legend\nTurbo King\nWilliams\n\n").expect("Write failed");

    file.write_all(b"NON-ALCOHOLIC\n").expect("Write failed");
    file.write_all(b"Maltina\nAmstel Malta\nMalta Gold\nFayrouz\n").expect("Write failed");

    println!("Nigerian Breweries file created successfully!");
}
