use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{metadata, File as StdFile, Metadata};
use std::io::{Read, Write};
use std::path::Path;
use bincode;
use idgenerator::*;
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    Pdf,
    Docx,
    Xls,
    Txt,
    Csv,
    Pptx,
    Jpg,
    Png,
    Unknown
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: i64,
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub created: NaiveDateTime,
    pub modified: Option<NaiveDateTime>,
    pub accessed: Option<NaiveDateTime>,
    pub owner: (i64, String, String),
    pub people_with_access: Vec<(i64, String, String)>,
    pub ipfs_hash: String,
    pub onchain_txn_id: String,
    pub download_permission: bool,
    pub description: Option<String>,
}

pub fn generate_id() -> i64 {
    let options: IdGeneratorOptions = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(6);
    let _ = IdInstance::init(options).unwrap_or_else(|err| {
        eprintln!("Erro ao inicializar gerador de ID: {err}");
        process::exit(1);
    });
    IdInstance::next_id()
}

fn load_files_from_file(path: &str) -> std::io::Result<Vec<File>> {
    let mut file: StdFile = match StdFile::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(Vec::new()),
    };
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    let decoded: Vec<File> = bincode::deserialize(&buffer).expect("Falha na desserialização de Vec<File>");
    Ok(decoded)
}

fn save_files_to_file(files: &Vec<File>, path: &str) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(files).expect("Falha na serialização de Vec<File>");
    let mut file: StdFile = StdFile::create(path)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn add_files_to_file(file: File, path: &str) -> std::io::Result<()> {
    let mut files: Vec<File> = load_files_from_file(path)?;
    files.push(file);
    save_files_to_file(&files, path)
}

fn generate_fake_hash(length: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

fn get_file_type(file_path: &str) -> FileType {
    let path: &Path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pdf") => FileType::Pdf,
        Some("docx") => FileType::Docx,
        Some("xls") => FileType::Xls,
        Some("txt") => FileType::Txt,
        Some("csv") => FileType::Csv,
        Some("pptx") => FileType::Pptx,
        Some("jpg") => FileType::Jpg,
        Some("png") => FileType::Png,
        _ => FileType::Unknown,
    }
}

fn get_file_size(file_path: &str) -> std::io::Result<u64> {
    let metadata: Metadata = metadata(file_path)?;
    Ok(metadata.len())
}

pub fn create_new_file(owner: (i64, String, String), name: String, path: &str) -> Result<(), String> {
    let owner_access: (i64, String, String) = owner.clone();
    let file_size: u64 = get_file_size(path).map_err(|e| e.to_string())?; 
    let file: File = File {
        id: generate_id(),
        name,
        file_type: get_file_type(path),
        size: file_size,
        created: Utc::now().naive_utc(),
        modified: None,
        accessed: None,
        owner,
        people_with_access: vec![owner_access],
        ipfs_hash: generate_fake_hash(46),
        onchain_txn_id: generate_fake_hash(64),
        download_permission: false,
        description: None,
    };
    add_files_to_file(file, path).map_err(|e| format!("Erro ao salvar o arquivo: {}", e))?;
    println!("Arquivo criado e salvo com sucesso.");
    Ok(())
}

pub fn get_all_files(path: &str) -> Result<Vec<File>, String> {
    load_files_from_file(path).map_err(|e| format!("Erro ao carregar arquivos: {}", e))
}

pub fn get_file(path: &str, file_id: i64) -> Result<File, String> {
    let files: Vec<File> = get_all_files(path)?;
    files.into_iter().find(|file| file.id == file_id).ok_or_else(|| "Arquivo não encontrado".to_string())
}

pub fn modify_file(path: &str, file_id: i64, updated_file: File) -> Result<(), String> {
    let mut files: Vec<File> = get_all_files(path)?;
    let file_index: usize = files.iter().position(|file| file.id == file_id).ok_or_else(|| "Arquivo não encontrado".to_string())?;
    files[file_index] = updated_file;
    save_files_to_file(&files, path).map_err(|e| format!("Erro ao salvar alterações: {}", e))
}

pub fn delete_file(path: &str, file_id: i64) -> Result<(), String> {
    let mut files: Vec<File> = get_all_files(path)?;
    let initial_length: usize = files.len();
    files.retain(|file| file.id != file_id);
    if files.len() == initial_length {
        return Err("Arquivo não encontrado".to_string());
    }
    save_files_to_file(&files, path).map_err(|e| format!("Erro ao salvar alterações: {}", e))
}
