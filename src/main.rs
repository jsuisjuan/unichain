use chrono::NaiveDate;
use unichain::*;

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
