use anyhow::Result;
use std::collections::HashMap;
use std::env;

fn main() -> Result<()> {
    // initialize accompanyment map
    let mut acc_map = HashMap::new();

    let mut reader = csv::Reader::from_path("accompanyments.csv")?;

    for result in reader.records() {
        let record = result?;
        acc_map.insert(record[0].to_string(), record[1].to_string());
        acc_map.insert(record[1].to_string(), record[0].to_string());
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide an item in order to retrieve an accompanyment");
        std::process::exit(1)
    }
    if let Some(item) = acc_map.get(&args[1]) {
        println!("{}", item);
    } else {
        println!("{} not found", args[1]);
    }
    Ok(())
}
