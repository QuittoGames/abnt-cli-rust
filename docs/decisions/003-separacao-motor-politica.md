# ADR 003 — Separação de Motor (template.tex) e Política ABNT (abnt-header.tex)

**Status:** Aceito
**Data:** 2026-09-01
**Contexto:** ABNT Engine — Commit 4

## Context

O `template.tex` continha **duas responsabilidades** misturadas:

1. **Motor estrutural** — Quarto + Pandoc + abntex2 + biblatex-abnt + VS Code dark theme + tabelas + helpers Pandoc.
2. **Política ABNT** — decisões específicas de como aplicar NBR 14724 a figuras, fontes, lista de ilustrações.

Mistura dessas responsabilidades gerou problemas:

- Dificuldade de auditar mudanças (mudança de política escondida em 281 linhas).
- Impossibilidade de A/B testar políticas ABNT sem recompilar tudo.
- Conflitos em merges (regiões do mesmo arquivo usadas para fins diferentes).

## Decision

**Separar** as responsabilidades em dois arquivos:

- `templetes/template.tex` — **motor estrutural** (Quarto/Pandoc/abntex2 + helpers).
- `templetes/abnt-header.tex` — **política ABNT** (NBR 14724, macros, configurações).

`template.tex` carrega `abnt-header.tex` via `\input{abnt-header}` antes de `\begin{document}`.

## Consequences

### Positivas

- ✅ Política ABNT tem arquivo dedicado de ~70 linhas (vs 281 linhas).
- ✅ Mudanças de política não tocam motor estrutural.
- ✅ Diff de mudanças ABNT é pequeno e revisável.
- ✅ Política pode ser customizada sem reescrever o motor.
- ✅ Cada ADR de política aponta para um arquivo específico.

### Negativas

- ❌ Mais um arquivo no diretório `templetes/` (mas organizado).
- ❌ `\input{abnt-header}` assume mesmo diretório (vs path absoluto).

### Mitigações

- O `\input` é relativo ao `template.tex`, então `abnt-header.tex` deve ficar em `templetes/`.
- Documentado em README e em comentário no topo de cada arquivo.

## Validation

- ✅ Commit 4 criado com sucesso.
- ✅ 13 testes Rust passam (Camada 1 inalterada).
- ⏳ Camada 2 (renderização) precisa de lualatex no ambiente.

## References

1. Regra 5 do sistema de Quitto (separation of concerns).
2. ADR 001 (bibliografia) e ADR 002 (norma ABNT) já apontam para essa arquitetura.
3. Padrão recomendado por: <https://www.ctan.org/pkg/abntex2> (modularidade via `\input`).
