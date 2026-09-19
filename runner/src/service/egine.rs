// egine.rs
// Responsabilidade: resolver o template ABNT e orquestrar `quarto render`.
// Módulo do pacote `service` (wiring: `mod service;` em main.rs -> service/mod.rs).

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::errors;
use crate::utils;

/// Resolve o caminho do template.tex usando prioridade:
///
///   ABNT_TEMPLATE > <repo>/templetes/template.tex > erro
pub fn resolve_template_path() -> Result<PathBuf, String> {
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

/// Resolve o diretório onde procurar `.bib` (o do .qmd / projeto).
fn project_dir_for(qmd_filename: &str) -> PathBuf {
    Path::new(qmd_filename)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Retorna o valor da env var `TEXINPUTS` para as chamadas do Quarto.
///
/// O novo formato ABNT do template (`templetes/template.tex`) faz
/// `\input{components/abnt-header}`, e o LaTeX resolve esses includes
/// relativo ao diretório de compilação (CWD do render), não ao diretório
/// do template. Este valor inclui o diretório do template na busca do
/// LaTeX, cobrindo `components/` (busca recursiva).
///
/// Sintaxe kpathsea: separador `;` (Windows), `//` = busca recursiva,
/// `;` final = anexa aos paths default (não os substitui).
fn texinputs_for(template_path: &Path) -> String {
    template_path
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
        .map(|dir| format!("{}//;", dir.to_string_lossy().replace('\\', "/")))
        // Sem diretório utilizável: ";" mantém apenas os paths default
        .unwrap_or_else(|| ";".to_string())
}

pub fn render_pdf(system_path: &str, qmd_filename: &str, template_path: &Path) -> ExitCode {
    // TEXINPUTS: inclui o diretório do template na busca do LaTeX
    // (novo formato ABNT: template.tex -> \input{components/...}).
    let texinputs = texinputs_for(template_path);

    // 1. Verificar se quarto está instalado
    let version_check = Command::new("quarto")
        .arg("--version")
        .env("PATH", system_path)
        .env("TEXINPUTS", &texinputs)
        .status();
    if version_check.is_err() {
        eprintln!("{}", errors::QUARTO_NOT_FOUND);
        return ExitCode::from(2);
    }

    // 2. Limpar cache
    let clear_status = Command::new("quarto")
        .args(["render", "--clear"])
        .env("PATH", system_path)
        .env("TEXINPUTS", &texinputs)
        .status();

    if let Ok(s) = clear_status {
        if !s.success() {
            eprintln!("{}", errors::QUARTO_RENDER_CLEAR_FAILED);
            return ExitCode::from(1);
        }
    }

    // 3. Construir args do render
    // Buscar .bib no diretório do .qmd (projeto), não no template
    let qmd_dir = project_dir_for(qmd_filename);
    let template_path_str = template_path
        .to_str()
        .expect("template path contains invalid UTF-8");
    let args = utils::environment::build(qmd_filename, &qmd_dir, template_path_str);

    // 4. Render
    let render_status = Command::new("quarto")
        .args(&args)
        .env("PATH", system_path)
        .env("TEXINPUTS", &texinputs)
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
