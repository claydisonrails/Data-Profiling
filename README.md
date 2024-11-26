# Data Profiler

Este é um projeto simples em Rust para analisar e gerar relatórios a partir de dados extraídos de arquivos CSV. O programa extrai dados das colunas de um CSV e gera relatórios no formato JSON ou Markdown.

## Funcionalidades

- Análise de arquivos CSV com dados numéricos e textuais.
- Geração de relatórios com estatísticas das colunas do CSV.
- Suporte para exportação do relatório em formato JSON ou Markdown.

## Como usar
- cargo run -- ./dados.csv markdown
- Como funciona
    O programa analisa as colunas de um arquivo CSV e calcula as estatísticas básicas (média, desvio padrão, etc.).
    Depois, o relatório é gerado no formato escolhido (JSON ou Markdown) e salvo no diretório atual.

### Requisitos

- Rust 1.60 ou superior instalado.
- Dependências do Rust configuradas corretamente no seu ambiente.

### Executando o Projeto

1. Clone o repositório:
   ```bash
   git clone https://github.com/claydisonrails/Data-Profiling
   cd data-profiler

2. Instale as dependências:
    cargo build

3. Execute o programa com um arquivo CSV:
    cargo run -- /caminho/para/seu/arquivo.csv formato
- Onde:
    /caminho/para/seu/arquivo.csv é o caminho para o arquivo CSV.
    formato pode ser json ou markdown, dependendo do formato que você deseja para o relatório.