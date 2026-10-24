use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efccrecords_merged.txt")
        .expect("Failed to create the file");

    for i in 0..commissioners.len() {
        let record = format!(
            "Commissioner: {}\nMinistry: {}\nZone: {}\n\n",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        println!("{}", record);

        file.write_all(record.as_bytes())
            .expect("Failed to write to file");
    }

    println!("Your EFCC records have been merged successfully, have a nice day!");
}