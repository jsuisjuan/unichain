use chrono::NaiveDate;
use idgenerator::*;
use std::process;

#[derive(Debug)]
enum FileType {
    Pdf,
    Docx,
    Xls,
    Txt,
    Csv,
    Pptx,
    Jpg,
    Png,
}

#[derive(Debug)]
struct File {
    id: i64,
    name: String,
    file_type: FileType,
    size: u64,
    created: NaiveDate,
    modified: NaiveDate,
    accessed: Option<NaiveDate>,
    owner: (i64, String, String),
    people_with_access: Vec<(i64, String, String)>,
    ipfs_hash: String,
    onchain_txn_id: String,
    download_permission: bool,
    description: Option<String>
}

fn generate_id() -> i64 {
    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(options).unwrap_or_else(|err| {
        eprintln!("Problem on instancing id generator options: {err}");
        process::exit(1);
    });
    IdInstance::next_id()
}

fn main() {
    println!("Welcome to your UniChain!");
    // log my personal data
    


    let file = File {
        id: generate_id(),
        name: String::from("Example Document"),
        file_type: FileType::Pdf,
        size: 2048,
        created: NaiveDate::from_ymd_opt(2024, 10, 26).expect("Invalid date"),
        modified: NaiveDate::from_ymd_opt(2024, 10, 26).expect("Invalid date"),
        accessed: None,
        owner: (generate_id(), String::from("John Doe"), String::from("john.doe@gmail.com")),
        people_with_access: vec![
            (generate_id(), String::from("Jane Smith"), String::from("jane.smith@gmail.com")),
            (generate_id(), String::from("Alice Johnson"), String::from("alice.johnson@outlook.com")),
        ],
        ipfs_hash: String::from("QmExampleIPFSHash"),
        onchain_txn_id: String::from("0xExampleBlockchainTxnID"),
        download_permission: true,
        description: Some(String::from("Sample file for testing")),
    };
    println!("{:#?}", file);
}
