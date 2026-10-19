//! Testes de coerência de manutenção da variante escolar.
//!
//! `templetes/template-canello-marques.tex` é uma cópia do template-base
//! `templetes/template.tex` com exatamente duas regiões de override
//! autorizadas (spec aprovada pelo DEV):
//!
//!   1. o bloco `capa` (capa do Colégio Canello Marques);
//!   2. `\setlength{\parskip}{0pt}` da seção PARÁGRAFOS → `6pt`,
//!      com comentários da divergência registrada.
//!
//! O teste de reconstrução garante que manutenção futura do template-base
//! (novas linhas, macros, pacotes) seja obrigatoriamente refletida na
//! variante — qualquer divergência fora das duas regiões quebra o teste.

use std::fs;
use std::path::PathBuf;

fn templates_dir() -> PathBuf {
    let manifest =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR definido pelo cargo");
    PathBuf::from(manifest).join("..").join("templetes")
}

fn lines_of(name: &str) -> Vec<String> {
    let path = templates_dir().join(name);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("falha ao ler {}: {e}", path.display()));
    content.lines().map(str::to_string).collect()
}

/// Primeira posição em `lines[from..]` cuja linha contém `needle`.
fn position_from(lines: &[String], needle: &str, from: usize) -> usize {
    lines[from.min(lines.len())..]
        .iter()
        .position(|line| line.contains(needle))
        .map(|p| p + from)
        .unwrap_or_else(|| panic!("linha contendo {needle:?} não encontrada a partir de {from}"))
}

/// Retorna o trecho de linhas [início, fim] inclusivo, buscando delimitadores.
fn block<'a>(lines: &'a [String], start_needle: &str, end_needle: &str) -> &'a [String] {
    let start = position_from(lines, start_needle, 0);
    let end = position_from(lines, end_needle, start);
    &lines[start..=end]
}

#[test]
fn variante_reconstruivel_a_partir_do_template_base() {
    let base = lines_of("template.tex");
    let variante = lines_of("template-canello-marques.tex");

    // Região 1 — bloco capa.
    let base_capa = block(
        &base,
        "% ---------------- CAPA ----------------",
        r"\end{capa}",
    );
    let variante_capa = block(
        &variante,
        "CAPA (override Colégio Canello Marques)",
        r"\end{capa}",
    );

    // Região 2 — parskip da seção PARÁGRAFOS (ancorado na linha de parindent
    // para não confundir com as ocorrências internas ao \tightlist).
    let base_parindent = position_from(&base, r"\setlength{\parindent}{1.25cm}", 0);
    let base_parskip = position_from(&base, r"\setlength{\parskip}{0pt}", base_parindent);
    let variante_parindent = position_from(&variante, r"\setlength{\parindent}{1.25cm}", 0);
    let variante_parskip_end =
        position_from(&variante, r"\setlength{\parskip}{6pt}", variante_parindent);
    let variante_parskip_block = &variante[variante_parindent + 1..=variante_parskip_end];

    // Reconstrói o template-base a partir da variante (e vice-versa é
    // equivalente): substitui as duas regiões de volta pelos valores base.
    // Ordem no arquivo: parskip (seção PARÁGRAFOS) vem ANTES da capa.
    let mut reconstruida: Vec<String> = Vec::with_capacity(variante.len());
    let base_capa_start = position_from(&base, "% ---------------- CAPA ----------------", 0);
    let base_capa_end = base_capa_start + base_capa.len() - 1;
    assert!(
        base_parskip < base_capa_start,
        "pré-condição do teste: parskip aparece antes da capa no template-base"
    );

    reconstruida.extend_from_slice(&base[0..=base_parindent]);
    reconstruida.extend_from_slice(variante_parskip_block);
    reconstruida.extend_from_slice(&base[base_parskip + 1..base_capa_start]);
    reconstruida.extend_from_slice(variante_capa);
    reconstruida.extend_from_slice(&base[base_capa_end + 1..]);

    assert_eq!(
        reconstruida, variante,
        "a variante template-canello-marques.tex divergiu do template-base \
         fora das duas regiões autorizadas (capa + parskip). Sincronize a \
         variante com o template-base ou amplie a spec com o DEV."
    );

    // O capa da variante é diferente do capa do base (senão a cópia perdeu o override).
    assert_ne!(
        variante_capa, base_capa,
        "o bloco capa da variante é idêntico ao do base — override perdido"
    );
}

#[test]
fn variante_preserva_contrato_da_capa_escolar() {
    let variante = lines_of("template-canello-marques.tex");
    let capa = block(
        &variante,
        "CAPA (override Colégio Canello Marques)",
        r"\end{capa}",
    );
    let capa_text = capa.join("\n");

    // Constante da escola e metadados permitidos (nenhum campo novo).
    assert!(capa_text.contains("COLÉGIO CANELLO MARQUES"));
    assert!(capa_text.contains("$title$"));
    assert!(capa_text.contains("$for(author)$"));
    assert!(capa_text.contains("$date$"));
    assert!(capa_text.contains("$if(cidade)$"));
    assert!(
        !capa_text.contains("$professor$"),
        "metadado novo $professor$ proibido pela spec"
    );
    assert!(
        !capa_text.contains("$instituicao$"),
        "metadado novo $instituicao$ proibido pela spec"
    );

    // Hierarquia de tamanhos exigida (escola/título 20 pt; autor/cidade 16 pt).
    assert!(capa_text.contains(r"\fontsize{20}{24}"));
    assert!(capa_text.contains(r"\fontsize{16}{20}"));
}

#[test]
fn parskip_da_variante_e_6pt_e_tightlist_intacto() {
    let variante = lines_of("template-canello-marques.tex");

    let parindent = position_from(&variante, r"\setlength{\parindent}{1.25cm}", 0);
    let parskip = position_from(&variante, r"\setlength{\parskip}{6pt}", parindent);
    assert!(
        parskip > parindent,
        "parskip 6pt deve estar na seção PARÁGRAFOS, após o parindent"
    );

    // As ocorrências internas ao \tightlist permanecem 0pt (não foram
    // tocadas). No template elas são multilinha e terminam com "%",
    // o que as distingue da linha solta da seção PARÁGRAFOS.
    let tightlist_refs = variante
        .iter()
        .filter(|line| line.contains(r"\setlength{\parskip}{0pt}%"))
        .count();
    assert_eq!(
        tightlist_refs, 2,
        "esperadas 2 ocorrências de parskip 0pt dentro dos \\tightlist"
    );
}
