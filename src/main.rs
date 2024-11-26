mod reader;
mod stats;
mod report;

use reader::read_csv;
use stats::calculate_stats;
use report::{generate_json_report, generate_markdown_report};

fn main() {
    let file_path = "sample.csv"; // Caminho para o arquivo CSV
    let data = match read_csv(file_path) { // Lê o CSV
        Ok(data) => data,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo CSV: {}", e);
            return;
        }
    };

    let stats = match calculate_stats(&data) { // Calcula as estatísticas dos dados
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Erro ao calcular estatísticas: {}", e);
            return;
        }
    };

    // Gera os relatórios em JSON e Markdown
    if let Err(e) = generate_json_report(&data, &stats) {
        eprintln!("Erro ao gerar relatório JSON: {}", e);
    }

    if let Err(e) = generate_markdown_report(&data, &stats) {
        eprintln!("Erro ao gerar relatório Markdown: {}", e);
    }
}
