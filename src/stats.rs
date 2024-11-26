#[derive(Debug)]
pub struct ColumnStats {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
}

#[derive(Debug)]
pub struct Stats {
    pub column_stats: Vec<ColumnStats>,
}

pub fn calculate_stats(data: &[Vec<String>]) -> Result<Stats, String> {
    if data.is_empty() || data[0].is_empty() {
        return Err("Nenhum dado disponível para análise".to_string());
    }

    let mut column_stats = Vec::new();

    for col_idx in 0..data[0].len() {
        let column_values: Vec<f64> = data.iter()
            .filter_map(|row| row[col_idx].parse::<f64>().ok())
            .collect();

        if column_values.is_empty() {
            column_stats.push(ColumnStats {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
            });
            continue;
        }

        let mean = column_values.iter().sum::<f64>() / column_values.len() as f64;
        let median = {
            let mut sorted = column_values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                (sorted[mid - 1] + sorted[mid]) / 2.0
            } else {
                sorted[mid]
            }
        };
        let variance = column_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / column_values.len() as f64;
        let std_dev = variance.sqrt();

        column_stats.push(ColumnStats { mean, median, std_dev });
    }

    Ok(Stats { column_stats })
}
