# ADR 001 — Sistema de Bibliografia: BibLaTeX vs Pandoc-CSL

**Status:** Aceito
**Data:** 2026-09-01
**Contexto:** ABNT Engine Runner — Commit 2

## Context

O projeto tinha dois caminhos bibliográficos ativos simultaneamente:

1. **BibLaTeX + biblatex-abnt** — usado pelo `template.tex` linhas 11-12.
2. **Pandoc-CSL** — referenciado pelo arquivo `csl/associacao-brasileira-de-normas-tecnicas.csl` (673 linhas, atualizado 2025-05-15).

Esses caminhos são **mutuamente exclusivos** no Quarto/Pandoc. Manter ambos gera ambiguidade e nenhum funciona corretamente.

## Decision

**MANTER BibLaTeX + biblatex-abnt. REMOVER CSL órfão.**

### Critérios avaliados

| Critério | BibLaTeX-abnt (A) | Pandoc-CSL (B) | Vencedor |
|---|---|---|---|
| Compatibilidade com abntex2 | ✅ Nativa (mesmo time) | ❌ Conflito | A |
| Compatibilidade com Quarto | ✅ Via `--metadata` | ✅ Nativa via YAML | Empate |
| Referências cruzadas | ✅ BibLaTeX hyperref | ✅ CSL citeproc | Empate |
| Citações no texto | ✅ `\cite{}` | ✅ Pandoc `[@key]` | Empate |
| Bibliografia ABNT (NBR 6023) | ✅ `style=abnt` (2002) | ✅ CSL mantido por comunidade | Empate |
| Manutenção | ✅ abntex2 ativo | ⚠️ CSL irregular | A |
| Determinismo | ✅ Total | ⚠️ Depende de CSL externo | A |
| Compatibilidade futura | ✅ Migrar = trocar .bst | ⚠️ CSL depende de mantenedor | A |
| Dependência externa | Biber (TeX Live) | Citeproc (Pandoc) | Empate |
| Comportamento Windows | ✅ Biber + TeX Live | ✅ Citeproc | Empate |
| Facilidade de testes | ✅ Saída previsível | ⚠️ CSL evolui | A |
| Já configurado no projeto | ✅ Linhas 11-12 | ❌ Órfão | A |

## Consequences

### Positivas

- ✅ Pipeline fica determinístico: Quarto → Pandoc → Biber → BibLaTeX → PDF.
- ✅ Compatível com abntex2 (mesma equipe de desenvolvimento).
- ✅ Suporte nativo a NBR 10520 (citações) e NBR 6023 (referências) via biblatex-abnt.
- ✅ Reduz dívida técnica (CSL nunca foi usado).

### Negativas

- ❌ Usuários que querem CSL precisam migrar manualmente (mas isso é raro no contexto acadêmico brasileiro).
- ❌ YAML `bibliography:` do Quarto não funciona diretamente — precisa `--metadata bibliography=...` (já implementado no runner Commit 1).

### Mitigações

- Manter BibLaTeX-abnt bem documentado.
- Documentar para usuários: "use \addbibresource{refs.bib} no template OU deixe o runner detectar automaticamente".

## Validation

- ✅ Cargo build passa.
- ✅ Cargo test passa (9 testes).
- ✅ `--metadata bibliography=refs.bib` é injetado quando há `.bib` único no diretório do template.
- ⏳ PDF real não validado (sem LaTeX engine no ambiente de Commit 1; será validado em Commit 3 com fixtures).
