use accounts::{PackedUserOperation, Safe7579, UserOpBuilder};
use clap::Parser;
use erc7579::types::{execute, ERC7579Account};
use std::path::PathBuf;
use tx_builder::parse_tx_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON file to parse
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    println!("Parsing file: {:?}", args.file);

    // Convert PathBuf to &str
    let file_path = args.file.to_str().expect("Invalid file path");

    // Parse the file
    match parse_tx_file(file_path) {
        Ok(data) => println!("Parsing data"),
        Err(e) => eprintln!("Error parsing file: {:?}", e),
    }

    let tx = parse_tx_file(file_path).unwrap();

    println!("{:#?}", tx);

    let call_data = execute(tx.transactions).unwrap();
    println!("execution call_data: {:?}", call_data);

    let safe = Safe7579;
    let user_op = safe.new(
        tx.meta.account_address.unwrap(),
        tx.meta.validator_module,
        call_data,
    );
    println!("{:#?}", user_op);
}
