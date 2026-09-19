// main.rs
// Entry point do ABNT runner.
// Responsabilidade: orquestrar `quarto render` recebendo um `.qmd`.
//
// Argumentos:
//   argv[1] = PATH do sistema (Windows: separador ';')
//   argv[2] = nome do arquivo .qmd
//
// Resolução do template (ordem de prioridade):
//   1. Env var ABNT_TEMPLATE (mais específica)
//   2. <runner_dir>/../templetes/template.tex (relativo ao binário)
//   3. Erro claro se nada for encontrado
//
// Formato ABNT novo: template.tex usa components/ (\input{components/...});
// o runner injeta TEXINPUTS (diretório do template) nas chamadas do quarto.
//
// Resolução do .bib:
//   - Busca automática no diretório do .qmd (projeto)
//   - Se encontrar exatamente 1 arquivo .bib, injeta --metadata bibliography=<caminho_absoluto>

use service::egine::{render_pdf, resolve_template_path};
use std::env;
use std::process::ExitCode;

mod service;
mod utils;

/// Mensagens de erro centralizadas (regra 11 — não duplicar).
mod errors {
    pub const QUARTO_NOT_FOUND: &str = "[ERROR] quarto not found in PATH";
    pub const QUARTO_RENDER_CLEAR_FAILED: &str = "[ERROR] quarto render --clear failed";
    pub const QUARTO_RENDER_FAILED: &str = "[ERROR] quarto render failed";
    pub const TEMPLATE_NOT_FOUND: &str = "[ERROR] template.tex not found. Set ABNT_TEMPLATE env var or place template at <repo>/templetes/template.tex";
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let system_path = match args.get(1).map(|s| s.as_str()) {
        Some(p) => p,
        None => {
            eprintln!("[ERROR] missing system PATH argument");
            return ExitCode::from(1);
        }
    };

    let qmd_filename = match args.get(2).map(|s| s.as_str()) {
        Some(f) => f,
        None => {
            eprintln!("[ERROR] missing qmd filename argument");
            return ExitCode::from(1);
        }
    };

    let template_path = match resolve_template_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(1);
        }
    };

    render_pdf(system_path, qmd_filename, &template_path)
}
