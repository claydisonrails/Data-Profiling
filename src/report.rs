use crate::stats::Stats;
use serde_json::json;
use std::fs;

pub fn generate_json_report(data: &[Vec<String>], stats: &Stats) -> Result<(), String> {
    let mut json_data = json!({
        "statistics": [],
        "data": data
    });

    let stats_json = stats.column_stats.iter().enumerate().map(|(i, stat)| {
        json!({
            "column": i + 1,
            "mean": stat.mean,
            "median": stat.median,
            "std_dev": stat.std_dev
        })
    }).collect::<Vec<_>>();

    json_data["statistics"] = json!(stats_json);

    fs::write("report.json", serde_json::to_string_pretty(&json_data).unwrap())
        .map_err(|e| format!("Erro ao salvar relatório JSON: {}", e))
}

pub fn generate_markdown_report(data: &[Vec<String>], stats: &Stats) -> Result<(), String> {
    let mut markdown = String::new();

    markdown.push_str("# Relatório de Estatísticas\n\n");

    for (i, column_stat) in stats.column_stats.iter().enumerate() {
        markdown.push_str(&format!("## Coluna {}\n", i + 1));
        markdown.push_str(&format!("- Média: {:.2}\n", column_stat.mean));
        markdown.push_str(&format!("- Mediana: {:.2}\n", column_stat.median));
        markdown.push_str(&format!("- Desvio Padrão: {:.2}\n\n", column_stat.std_dev));
    }

    markdown.push_str("## Dados Originais\n\n");
    for row in data {
        markdown.push_str(&format!("| {} |\n", row.join(" | ")));
    }

    fs::write("report.md", markdown).map_err(|e| format!("Erro ao salvar relatório Markdown: {}", e))
}
