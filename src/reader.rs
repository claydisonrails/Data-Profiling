use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, String> {
    let path = Path::new(file_path);
    let file = File::open(&path).map_err(|e| format!("Erro ao abrir arquivo: {}", e))?;

    let mut data = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("Erro ao ler linha: {}", e))?;
        let row = line.split(';').map(|v| v.to_string()).collect::<Vec<_>>();
        data.push(row);
    }

    if data.is_empty() {
        return Err("Arquivo vazio ou inválido".to_string());
    }

    Ok(data)
}
