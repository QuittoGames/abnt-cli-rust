// environment.rs
// Responsabilidade: descobrir arquivos do diretório do template sem chamar
// `cmd /C` (Windows-only, sujeito a command injection se paths vierem de CLI).
// Toda descoberta é feita em Rust puro (cross-platform, deterministic).

use std::fs;
use std::path::Path;

/// Tenta descobrir um único arquivo `.bib` dentro de `dir`.
///
/// Retorna `Some(filename)` se encontrar exatamente um arquivo,
/// `None` se encontrar zero ou mais de um (decisão de projeto: evita
/// ambiguidade silenciosa).
///
/// Args:
///   - `dir`: diretório onde procurar. **Não** é um caminho de template
///     nem o PATH do sistema — apenas um diretório.
///
/// Erros são tratados explicitamente: retorna `None` em qualquer falha de I/O.
pub fn discover_bib_in(dir: &Path) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;

    let bibs: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let name = path.file_name()?.to_str()?.to_string();
            if name.to_lowercase().ends_with(".bib") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    match bibs.len() {
        1 => Some(bibs.into_iter().next().expect("len==1 just verified")),
        _ => None, // 0 ou >1: comportamento conservador — não injeta nada
    }
}

/// Retorna os args `--metadata bibliography=<file>` se houver um único `.bib`
/// no diretório, ou `vec![]` caso contrário.
///
/// Args:
///   - `dir`: diretório onde procurar (geralmente o mesmo do template).
pub fn get_bibliografy(dir: &Path) -> Vec<String> {
    match discover_bib_in(dir) {
        Some(bib) => vec!["--metadata".to_string(), format!("bibliography={}", bib)],
        None => vec![],
    }
}

/// Retorna `["--template", <template_path>]` se o template existir,
/// `vec![]` caso contrário.
///
/// Args:
///   - `template_path`: caminho completo para um arquivo `.tex`.
pub fn get_template(template_path: &str) -> Vec<String> {
    if Path::new(template_path).is_file() {
        vec!["--template".to_string(), template_path.to_string()]
    } else {
        vec![]
    }
}

/// Constrói os args do `quarto render` para um dado `.qmd`.
///
/// Ordem dos args (importante para Quarto CLI):
///   1. `render <filename.qmd>`
///   2. `--metadata bibliography=<file>` (se houver .bib único)
///   3. `--template <path>` (se template existir)
pub fn build(qmd_filename: &str, template_dir: &Path, template_path: &str) -> Vec<String> {
    let mut args: Vec<String> = vec!["render".to_string(), qmd_filename.to_string()];

    for bib_arg in get_bibliografy(template_dir) {
        args.push(bib_arg);
    }
    for tpl_arg in get_template(template_path) {
        args.push(tpl_arg);
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;

    fn make_temp_dir(name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("abnt_test_{}_{}", name, std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn discover_bib_returns_none_when_no_bib() {
        let dir = make_temp_dir("no_bib");
        assert_eq!(discover_bib_in(&dir), None);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_returns_some_when_one_bib() {
        let dir = make_temp_dir("one_bib");
        File::create(dir.join("refs.bib")).unwrap();
        assert_eq!(discover_bib_in(&dir), Some("refs.bib".to_string()));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_returns_none_when_multiple_bibs() {
        // Decisão de projeto: >1 .bib = ambiguidade, não injeta nada.
        let dir = make_temp_dir("multi_bib");
        File::create(dir.join("a.bib")).unwrap();
        File::create(dir.join("b.bib")).unwrap();
        assert_eq!(discover_bib_in(&dir), None);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_is_case_insensitive() {
        let dir = make_temp_dir("case");
        File::create(dir.join("REFS.BIB")).unwrap();
        assert_eq!(discover_bib_in(&dir), Some("REFS.BIB".to_string()));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn get_bibliografy_returns_metadata_args_when_bib_found() {
        let dir = make_temp_dir("meta");
        File::create(dir.join("refs.bib")).unwrap();
        let args = get_bibliografy(&dir);
        assert_eq!(args, vec!["--metadata", "bibliography=refs.bib"]);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn get_template_returns_empty_when_file_missing() {
        let args = get_template("/nonexistent/path/template.tex");
        assert!(args.is_empty());
    }

    #[test]
    fn get_template_returns_args_when_file_exists() {
        let dir = make_temp_dir("tpl");
        let tpl = dir.join("template.tex");
        File::create(&tpl).unwrap();
        let args = get_template(tpl.to_str().unwrap());
        assert_eq!(
            args,
            vec!["--template".to_string(), tpl.to_str().unwrap().to_string()]
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn build_combines_render_bib_and_template() {
        let dir = make_temp_dir("build");
        File::create(dir.join("refs.bib")).unwrap();
        let tpl = dir.join("template.tex");
        File::create(&tpl).unwrap();

        let args = build("doc.qmd", &dir, tpl.to_str().unwrap());
        assert_eq!(args[0], "render");
        assert_eq!(args[1], "doc.qmd");
        assert!(args.contains(&"--metadata".to_string()));
        assert!(args.contains(&"bibliography=refs.bib".to_string()));
        assert!(args.contains(&"--template".to_string()));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn build_works_without_bib_or_template() {
        let dir = make_temp_dir("empty_build");
        let args = build("doc.qmd", &dir, "/nonexistent/template.tex");
        assert_eq!(args, vec!["render", "doc.qmd"]);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_ignores_directories_ending_in_bib() {
        // Edge case: diretório com nome "fake.bib" não deve ser contado como .bib.
        let dir = make_temp_dir("dir_with_bib_suffix");
        fs::create_dir(dir.join("fake.bib")).unwrap();
        File::create(dir.join("real.bib")).unwrap();
        let result = discover_bib_in(&dir);
        assert_eq!(result, Some("real.bib".to_string()));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_supports_files_with_dashes_and_underscores() {
        // Nomes válidos para .bib: refs.bib, minhas-refs.bib, refs_2026.bib.
        let dir = make_temp_dir("dashes");
        File::create(dir.join("minhas-refs_2026.bib")).unwrap();
        assert_eq!(
            discover_bib_in(&dir),
            Some("minhas-refs_2026.bib".to_string())
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn discover_bib_returns_none_for_nonexistent_dir() {
        // Não deve panic; retorna None para diretório inexistente.
        let dir = PathBuf::from("/nonexistent/dir/that/does/not/exist");
        assert_eq!(discover_bib_in(&dir), None);
    }

    #[test]
    fn discover_bib_ignores_non_bib_files() {
        // Arquivos .txt, .tex, .qmd não devem ser contados.
        let dir = make_temp_dir("mixed");
        File::create(dir.join("template.tex")).unwrap();
        File::create(dir.join("document.qmd")).unwrap();
        File::create(dir.join("notes.txt")).unwrap();
        File::create(dir.join("refs.bib")).unwrap();
        assert_eq!(discover_bib_in(&dir), Some("refs.bib".to_string()));
        fs::remove_dir_all(&dir).ok();
    }
}
