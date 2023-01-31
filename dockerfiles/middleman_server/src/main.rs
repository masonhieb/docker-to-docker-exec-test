use anyhow::Result;
use serde::Deserialize;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please enter an item in order to get its accompanyment");
        std::process::exit(1);
    }

    let res: ServerResponse = reqwest::blocking::get(format!(
        "http://localhost:3000/accompanyment?item={}",
        args[1]
    ))?
    .json()?;

    println!("{}", res.item);

    Ok(())
}

#[derive(Deserialize)]
struct ServerResponse {
    item: String,
}
