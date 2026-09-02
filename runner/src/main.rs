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

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

mod utils;

/// Mensagens de erro centralizadas (regra 11 — não duplicar).
mod errors {
    pub const QUARTO_NOT_FOUND: &str = "[ERROR] quarto not found in PATH";
    pub const QUARTO_RENDER_CLEAR_FAILED: &str = "[ERROR] quarto render --clear failed";
    pub const QUARTO_RENDER_FAILED: &str = "[ERROR] quarto render failed";
    pub const TEMPLATE_NOT_FOUND: &str = "[ERROR] template.tex not found. Set ABNT_TEMPLATE env var or place template at <repo>/templetes/template.tex";
}

/// Resolve o caminho do template.tex usando prioridade:
///
///   ABNT_TEMPLATE > <repo>/templetes/template.tex > erro
fn resolve_template_path() -> Result<PathBuf, String> {
    // 1. Env var ABNT_TEMPLATE (se não-vazia)
    if let Ok(p) = env::var("ABNT_TEMPLATE") {
        if !p.trim().is_empty() {
            let path = PathBuf::from(&p);
            if path.is_file() {
                return Ok(path);
            }
            return Err(format!(
                "ABNT_TEMPLATE points to '{}' but file does not exist",
                p
            ));
        }
    }

    // 2. Fallback relativo ao executável
    //    <exe_dir>/../templetes/template.tex
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Caminho do repo: <runner/target/release/..> -> <repo>/templetes
            // Subir 3 níveis: release/ -> debug or target/ -> runner/ -> repo/
            // Mas queremos só 1 nível para debug local; o runner roda em dev
            // a partir de <repo>/runner/.
            let candidate = exe_dir
                .join("..") // runner/target/release/.. -> runner/target/
                .join("..") // runner/target/.. -> runner/
                .join("..") // runner/.. -> repo/
                .join("templetes")
                .join("template.tex");

            if candidate.is_file() {
                return Ok(candidate);
            }

            // Alternativa: <runner_dir>/../templetes (quando rodando do source
            // com `cargo run`, exe está em target/debug/, então ../../templetes
            // cai em <repo>/templetes).
            let candidate_alt = exe_dir
                .join("..")
                .join("..")
                .join("templetes")
                .join("template.tex");

            if candidate_alt.is_file() {
                return Ok(candidate_alt);
            }
        }
    }

    Err(errors::TEMPLATE_NOT_FOUND.to_string())
}

/// Resolve o diretório onde procurar `.bib` (geralmente, o do template).
fn bib_dir_for(template_path: &Path) -> PathBuf {
    template_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn render_pdf(system_path: &str, qmd_filename: &str, template_path: &Path) -> ExitCode {
    // 1. Verificar se quarto está instalado
    let version_check = Command::new("quarto")
        .arg("--version")
        .env("PATH", system_path)
        .status();
    if version_check.is_err() {
        eprintln!("{}", errors::QUARTO_NOT_FOUND);
        return ExitCode::from(2);
    }

    // 2. Limpar cache
    let clear_status = Command::new("quarto")
        .args(["render", "--clear"])
        .env("PATH", system_path)
        .status();
    if let Ok(s) = clear_status {
        if !s.success() {
            eprintln!("{}", errors::QUARTO_RENDER_CLEAR_FAILED);
            return ExitCode::from(1);
        }
    }

    // 3. Construir args do render
    let template_dir = bib_dir_for(template_path);
    let template_path_str = template_path
        .to_str()
        .expect("template path contains invalid UTF-8");
    let args = utils::environment::build(qmd_filename, &template_dir, template_path_str);

    // 4. Render
    let render_status = Command::new("quarto")
        .args(&args)
        .env("PATH", system_path)
        .status();

    match render_status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(_) => {
            eprintln!("{}", errors::QUARTO_RENDER_FAILED);
            ExitCode::from(1)
        }
        Err(_) => {
            eprintln!("{}", errors::QUARTO_NOT_FOUND);
            ExitCode::from(2)
        }
    }
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
