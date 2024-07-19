use clap::Parser;
use std::path::PathBuf;
use tx_builder::parse_tx_file;
use erc7579::types::ERC7579Account;

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
        Ok(data) => println!("Parsed data: {:#?}", data),
        Err(e) => eprintln!("Error parsing file: {:?}", e),
    }

    let data = parse_tx_file("./examples/example.json");
    println!("{:#?}", data);



}


