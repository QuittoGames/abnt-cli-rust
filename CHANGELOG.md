# Changelog — ABNT Engine

Histórico de mudanças, em ordem cronológica inversa.

Formato baseado em [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased] — 2026-09-01

Plano atômico de 8 commits concluído. ABNT Engine pronto para produção.

### Commits 1-8 (2026-09-01)

#### Commit 8 — Documentação Final + Cleanup

- ✅ ADR 005 (resultado final dos 8 commits).
- ✅ `docs/specs/abnt-engine-architecture.md` (arquitetura consolidada).
- ✅ `CHANGELOG.md` (este arquivo).
- ✅ Status final adicionado a `docs/research/abnt-image-injection-research.md`.
- ✅ README revisado.

#### Commit 7 — Crossref + Listas + Bug B7

- ✅ **Bug B7 corrigido:** `\addbibresource{bib/bibliografia_quarto.bib}` hardcoded
  → `$if(bibliography)$\addbibresource{$bibliography$}$endif$`.
- ✅ Listas pré-textuais automáticas no template (sumário → ilustrações → tabelas).
- ✅ Macro renomeada: `\listafilustracoes` → `\listadeilustracoes`.
- ✅ Fixture 04 atualizada.
- ✅ ADR 004 (listas pré-textuais automáticas).

#### Commit 6 — Tabelas, Equações, Listagens

- ✅ Macro `\listadetabelas` (NBR 14724 §4.2.1.8).
- ✅ Macro `\listadequadros` (NBR 14724 §4.2.1.10, reservado).
- ✅ Macro `\tabelaIBGE{Título}{Fonte}{Conteúdo}`.
- ✅ `\captionsetup[table]` e `\captionsetup[listing]` ABNT.
- ✅ Fixture 12 (tabela ABNT).
- ✅ Fixture 13 (equação + listagem).

#### Commit 5 — Variáveis Dinâmicas (cidade)

- ✅ **Bug B8 corrigido:** "São Paulo" hardcoded → `$if(cidade)$$cidade$$else$São Paulo$endif$`.
- ✅ Fixture 11 (override de cidade).

#### Commit 4 — Política ABNT para Figuras

- ✅ `templetes/abnt-header.tex` criado (política separada).
- ✅ `\input{abnt-header}` adicionado a `template.tex` (+5 linhas).
- ✅ Macros `\fonteElaborado`, `\fonteAdaptado`, `\fonteRetirado`, `\fonteTraduzido`.
- ✅ Macro `\listadeilustracoes` (renomeada em Commit 7).
- ✅ Pacote `subcaption` para subfiguras.
- ✅ Fixtures 04-06 atualizadas para usar macros.
- ✅ ADR 003 (separação motor/política).

#### Commit 3 — Baseline de Testes Visuais

- ✅ 10 fixtures `.qmd` criadas (01-10).
- ✅ 4 imagens placeholder geradas (200x150 a 1200x900).
- ✅ `runner/tests/fixtures/smoke-test.ps1` (Camada 2).
- ✅ `docs/specs/testing-strategy.md` (6 camadas).
- ✅ `runner/tests/fixtures/README.md` (doc das fixtures).

#### Commit 2 — Resolver Ambiguidade Bibliográfica

- ✅ ADR 001 (BibLaTeX vs CSL: BibLaTeX mantido).
- ✅ ADR 002 (NBR 14724:2011 + legenda 12pt / fonte 10pt).
- ✅ CSL órfão movido para `Legacy/`.
- ✅ Fixture `.bib` com 3 entradas.
- ✅ `t.qmd` atualizado (removido `::: {#refs}`).
- ✅ README reescrito.
- ✅ +4 testes edge cases (total: 13).

#### Commit 1 — Corrigir Runner Rust

- ✅ **Bug B1 corrigido:** `abnt.bat` caminho hardcoded → `%~dp0\runner`.
- ✅ **Bug B3 corrigido:** `main.rs` template_path hardcoded → env var + fallback.
- ✅ **Bug B4 corrigido:** `utils/mod.rs` PascalCase morto removido.
- ✅ **Bug B5 corrigido:** `cmd /C dir | findstr` → `std::fs::read_dir`.
- ✅ **Bug B6 corrigido:** `is_empty()` validação adicionada.
- ✅ Runner Rust reescrito (cross-platform, exit codes corretos).
- ✅ 9 testes unitários adicionados.

## Estado Inicial (Pré-Commit 1)

Antes do plano atômico, o ABNT Engine tinha:

- ❌ `abnt.bat` com caminho hardcoded `D:\Research\ABNT\runner` (não-portável).
- ❌ `main.rs` com `template_path` hardcoded `D:\Research\ABNT\templetes\template.tex`.
- ❌ `utils/mod.rs` com `pub mod Environment` (PascalCase morto).
- ❌ `environment.rs` usando `cmd /C dir | findstr ".bib"` (Windows-only, command injection).
- ❌ `environment.rs` sem validação de `.bib` vazio.
- ❌ `template.tex` linha 12 com `\addbibresource{bib/bibliografia_quarto.bib}` (path inexistente).
- ❌ `template.tex` linhas 247, 265 com "São Paulo" hardcoded.
- ❌ CSL órfão em `csl/`.
- ❌ `::: {#refs}` (sintaxe CSL) em `t.qmd`.
- ❌ Política ABNT misturada com motor estrutural.
- ❌ Sem testes.
- ❌ Sem fixtures.
- ❌ Sem documentação de decisões (ADRs).
- ❌ Sem estratégia de testes.
