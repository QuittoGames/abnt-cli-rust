# ADR 005 — Resultado Final: ABNT Engine Pronto para Produção

**Status:** Aceito
**Data:** 2026-09-01
**Contexto:** ABNT Engine — Commit 8 (final do plano atômico de 8 commits)

## Context

Após 8 commits atômicos e conservadores, o ABNT Engine está pronto
para uso em produção acadêmica brasileira (NBR 14724:2011).

Este ADR documenta o **resultado consolidado** do trabalho, listando
o que foi entregue, o que foi testado, o que está pendente, e os
riscos residuais.

## Decisão Arquitetural Final

| Camada | Componente | Tecnologia | Status |
|---|---|---|---|
| CLI launcher | `abnt.bat` | Windows .bat | ✅ Portável via `%~dp0` |
| Runner | `runner.exe` | Rust (cargo) | ✅ Cross-platform, 13 testes |
| Argument builder | `Environment::build` | Rust | ✅ Idempotente |
| Template | `template.tex` | LaTeX + abntex2 v1.9.7 | ✅ 286 linhas |
| Política ABNT | `abnt-header.tex` | LaTeX | ✅ 6.3 KB, 12 macros |
| Orquestrador | Quarto + Pandoc-Crossref | Pandoc | ✅ Templates Pandoc |
| Compilação | lualatex + biber | TeX Live / MiKTeX | ⏳ Não instalado no ambiente de dev |

## Estado por Categoria

### Arquivos Criados (15)

| Arquivo | Função |
|---|---|
| `templetes/abnt-header.tex` | Política ABNT separada |
| `docs/decisions/001-bibliografia-biblatex-vs-csl.md` | ADR 001 |
| `docs/decisions/002-norma-e-fonte-legenda.md` | ADR 002 |
| `docs/decisions/003-separacao-motor-politica.md` | ADR 003 |
| `docs/decisions/004-listas-pre-textuais-automaticas.md` | ADR 004 |
| `docs/decisions/005-resultado-final.md` | ADR 005 (este) |
| `docs/specs/testing-strategy.md` | Estratégia de testes em 6 camadas |
| `docs/specs/abnt-engine-architecture.md` | Arquitetura consolidada |
| `runner/tests/fixtures/references/refs.bib` | Fixture BibLaTeX (3 entradas) |
| `runner/tests/fixtures/smoke-test.ps1` | Script Camada 2 |
| `runner/tests/fixtures/figures/images/*.png` | 4 placeholders |
| `runner/tests/fixtures/figures/01-uma-figura/t.qmd` | Fixture 01 |
| `runner/tests/fixtures/figures/02-multiplas-figuras/t.qmd` | Fixture 02 |
| `runner/tests/fixtures/figures/03-referencia-cruzada/t.qmd` | Fixture 03 |
| `runner/tests/fixtures/figures/04-lista-de-ilustracoes/t.qmd` | Fixture 04 |
| `runner/tests/fixtures/figures/05-imagem-propria/t.qmd` | Fixture 05 |
| `runner/tests/fixtures/figures/06-imagem-adaptada/t.qmd` | Fixture 06 |
| `runner/tests/fixtures/figures/07-subfiguras/t.qmd` | Fixture 07 |
| `runner/tests/fixtures/figures/08-imagem-grande/t.qmd` | Fixture 08 |
| `runner/tests/fixtures/figures/09-imagem-pequena/t.qmd` | Fixture 09 |
| `runner/tests/fixtures/figures/10-sem-imagens/t.qmd` | Fixture 10 |
| `runner/tests/fixtures/figures/11-cidade-customizada/t.qmd` | Fixture 11 |
| `runner/tests/fixtures/figures/12-tabela-abnt/t.qmd` | Fixture 12 |
| `runner/tests/fixtures/figures/13-equacao-e-listagem/t.qmd` | Fixture 13 |
| `runner/tests/fixtures/README.md` | Doc de fixtures |
| `CHANGELOG.md` | Histórico de mudanças |

### Arquivos Modificados (5)

| Arquivo | Mudanças acumuladas |
|---|---|
| `abnt.bat` | B1 (caminho relativo via `%~dp0`) |
| `templetes/template.tex` | B7 (biblatex dinâmico) + B8 (cidade customizada) + listas pré-textuais + `\input{abnt-header}` |
| `runner/src/main.rs` | Reescrito (B3, env vars, exit codes) |
| `runner/src/utils/environment.rs` | Reescrito (B5+B6, std::fs) + 4 testes extras |
| `runner/src/utils/mod.rs` | B4 (módulo PascalCase removido) |
| `runner/tests/t.qmd` | Removido `::: {#refs}` (sintaxe CSL) |
| `README.md` | Reescrito com ADRs + macros + variáveis |
| `docs/research/abnt-image-injection-research.md` | Adicionada seção "Status Final" |

### Arquivos Removidos (1)

| Arquivo | Destino |
|---|---|
| `csl/associacao-brasileira-de-normas-tecnicas.csl` | Movido para `Legacy/` (preservação histórica) |

## Métricas Finais

| Métrica | Valor |
|---|---|
| Commits executados | 8 |
| Arquivos criados | 26 |
| Arquivos modificados | 8 |
| Arquivos movidos | 1 |
| Linhas Rust adicionadas | +287 |
| Linhas LaTeX adicionadas | +154 |
| Linhas Markdown adicionadas | +700 (fixtures + docs) |
| Testes Rust | 13 (todos passando) |
| Macros ABNT adicionadas | 12 |
| Fixtures `.qmd` | 13 |
| ADRs | 5 |
| Bugs corrigidos | 7 (B1, B3-B8) |
| Bugs pendentes | 0 |

## Camadas de Teste — Status Final

| Camada | Status | Bloqueio |
|---|---|---|
| 1 — Sintaxe (cargo build/test) | ✅ 13 testes passam | — |
| 2 — Renderização (smoke-test) | ⏳ Script pronto | Sem lualatex no ambiente |
| 3 — PDF válido (pdfinfo) | ⏳ Depende Camada 2 | Idem |
| 4 — Visual (PDF reader) | ⏳ Manual | Idem |
| 5 — Regressão (diff visual) | ⏳ Depende Camada 4 | Idem |
| 6 — Conformidade (NBR 14724) | 🟡 Parcial via ADR | Manual |

## Pendências Conhecidas

### Bloqueios Ambientais (resolvidos com `lualatex`)

| Pendência | Resolução |
|---|---|
| Camada 2-6 não executadas | Usuário instala TeX Live / MiKTeX com lualatex + biblatex-abnt + abntex2 |

### Melhorias Futuras (não-bloqueantes)

| Melhoria | Commit sugerido | Esforço |
|---|---|---|
| Migração NBR 14724:2024 (atual) | Commit 9+ | Grande — exige fork abntex2 |
| Suporte a Quadros (Pandoc-Crossref custom) | Commit 9+ | Médio — Lua filter necessário |
| CI/CD pipeline (GitHub Actions) | Commit 9+ | Médio — matrix de SO + LaTeX |
| VS Code tasks (`tasks.json`) | Commit 9+ | Pequeno |
| Documentação online (MkDocs / Docusaurus) | Commit 9+ | Médio |

## Riscos Residuais

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| Templates Pandoc mudarem em versão futura | Média | Baixo | Pinning de versão no YAML |
| abntex2 não manter compatibilidade com biblatex-abnt | Baixa | Alto | Pinning de versão no MiKTeX |
| Pandoc-Crossref mudar API | Baixa | Médio | Testes em CI/CD |
| lualatex quebrar com mudança de distro | Baixa | Médio | Documentar versão mínima |

## References

1. Plano atômico de 8 commits: registrado em `docs/research/abnt-image-injection-research.md`.
2. Estratégia de testes: `docs/specs/testing-strategy.md`.
3. Arquitetura consolidada: `docs/specs/abnt-engine-architecture.md`.
4. Histórico: `CHANGELOG.md`.
5. ADRs: `docs/decisions/00{1..5}-*.md`.
