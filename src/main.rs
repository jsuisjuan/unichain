use chrono::NaiveDate;

#[derive(Debug)]
enum DocumentStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview
}

#[derive(Debug)]
struct TransferDocument {
    doc_name: String,
    student_id: u32,
    institution_origin: String,
    institution_destination: String,
    pages: u32,
    issued_at: NaiveDate,
    status: DocumentStatus
}

fn main() {
    println!("Welcome to the UniChain!");
    let issued_at: NaiveDate = NaiveDate::from_ymd_opt(2024, 10, 26)
        .expect("Invalid Date");
    let document: TransferDocument = TransferDocument {
        doc_name: String::from("Histórico Acadêmico"),
        student_id: 12345,
        institution_origin: String::from("Universidade A"),
        institution_destination: String::from("Universidade B"),
        pages: 10,
        issued_at,
        status: DocumentStatus::Approved
    };
    println!("{:#?}", document);
}
