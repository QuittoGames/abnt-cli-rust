# ABNT Engine — Arquitetura Consolidada

Documento final pós-Commit 8. Descreve a arquitetura completa do ABNT Engine.

## Overview

ABNT Engine é um wrapper CLI que automatiza a renderização de documentos
Markdown/Pandoc (`.qmd`) em PDFs no padrão ABNT brasileiro (NBR 14724:2011)
usando LuaLaTeX + abntex2 + biblatex-abnt.

## Goals and Non-Goals

### Goals

- ✅ Renderizar `.qmd` em PDF ABNT-compliant via Quarto.
- ✅ Detectar automaticamente template LaTeX e bibliografia BibLaTeX.
- ✅ Suportar Windows (via .bat) e Linux/macOS (via cargo direto).
- ✅ Documentação ABNT-compliant out-of-the-box (sem configuração).
- ✅ Testes unitários + smoke test prontos para CI/CD.

### Non-Goals

- ❌ Edição visual de template (WYSIWYG).
- ❌ Geração de documentos que NÃO sejam `.qmd` (`.tex`, `.docx`, etc.).
- ❌ Multi-norma (suporte apenas a NBR 14724:2011 — não 2024).
- ❌ Publicação em servidor (CLI local apenas).

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                     ABNT Engine                           │
├──────────────────────────────────────────────────────────┤
│                                                            │
│  [abnt.bat] ─→ compila runner se necessário ─→ [runner]   │
│       ↓                                                   │
│  ┌────────────────────────────────────────────────────┐   │
│  │ runner.exe (Rust)                                   │   │
│  │   ├─ main.rs    → CLI parser                       │   │
│  │   └─ utils/                                        │   │
│  │      └─ environment.rs → arg builder + .bib disco │   │
│  └────────────────────────────────────────────────────┘   │
│       ↓                                                   │
│  Quarto CLI (chamada externa)                              │
│       ↓                                                   │
│  Pandoc (processamento .qmd → .tex)                       │
│       ↓                                                   │
│  LuaLaTeX (compilação .tex → .pdf)                        │
│       ↓                                                   │
│  biblatex-abnt (bibliografia ABNT)                       │
│       ↓                                                   │
│  abntex2 (formatação ABNT)                                │
│       ↓                                                   │
│  biber (processamento .bib)                               │
│       ↓                                                   │
│  [PDF ABNT-compliant]                                     │
│                                                            │
└──────────────────────────────────────────────────────────┘
```

## Modules

### `runner/`

CLI binary em Rust. Detecta template + .bib, constrói args para Quarto.

**`runner/src/main.rs`** — CLI entry.
- Resolve `template_path` via env var `ABNT_TEMPLATE` ou fallback relativo.
- Detecta `.bib` único no diretório do template.
- Constrói args para Quarto: `render <file> --metadata bibliography=... --template ...`.
- Executa Quarto via `Command::new("quarto")`.
- Exit code: 0 = sucesso, 1 = falha.

**`runner/src/utils/environment.rs`** — Lógica de detecção.
- `discover_bib_in(dir) -> Option<String>` — busca `.bib` único (case-insensitive).
- `get_template(path) -> Option<String>` — verifica se template existe.
- `get_bibliografy(dir) -> Option<String>` — wrapper sobre `discover_bib_in`.
- `build(qmd, template_dir, template_path) -> Vec<String>` — constrói args Quarto.
- 13 testes unitários.

### `templetes/`

Templates LaTeX.

**`templetes/template.tex`** — Motor estrutural (286 linhas).
- Carrega `abntex2` (classe ABNT).
- Carrega `biblatex` com `style=abnt`.
- Setup de idioma (babel + fontspec).
- Setup de geometria, paginação, parágrafos.
- VS Code dark theme para código (override Pandoc Highlighting).
- Helpers Pandoc (tightlist, LTcaptype, CSLReferences removidas em Commit 2).
- Carrega `\input{abnt-header}` (Commit 4).
- Estrutura pré-textual: capa, folha de rosto, sumário, listas pré-textuais, conteúdo, bibliografia.

**`templetes/abnt-header.tex`** — Política ABNT (140 linhas).
- Pacotes: `subcaption` para subfiguras.
- Macros de fonte: `\fonteElaborado`, `\fonteAdaptado`, `\fonteRetirado`, `\fonteTraduzido`.
- Listas pré-textuais: `\listadeilustracoes`, `\listadetabelas`, `\listadequadros`.
- Tabela ABNT: `\tabelaIBGE` + `\captionsetup[table]`.
- Equação: ambiente LaTeX nativo.
- Listagem: `\captionsetup[listing]`.

### `runner/tests/fixtures/`

13 fixtures `.qmd` + 4 imagens placeholder.

### `docs/`

- `docs/decisions/` — 5 ADRs.
- `docs/research/` — relatório de pesquisa original.
- `docs/specs/` — estratégia de testes + arquitetura.
- `docs/ai/` — workspace AI (não usado).

## Domain Model

```
Documento ABNT
├── Metadata
│   ├── title (string)
│   ├── author (string)
│   ├── date (date)
│   ├── cidade (string, default "São Paulo")
│   └── bibliography (path, opcional)
├── Pré-textual
│   ├── Capa
│   ├── Folha de rosto
│   ├── Sumário
│   ├── Lista de ilustrações (sempre gerada)
│   └── Lista de tabelas (sempre gerada)
├── Conteúdo (textual)
│   ├── Capítulos
│   ├── Figuras (com fonte ABNT)
│   ├── Tabelas (com fonte ABNT)
│   ├── Equações (numeradas)
│   └── Listagens de código
└── Pós-textual
    └── Referências (BibLaTeX-abnt)
```

## Data Flow

```
1. Usuário executa: abnt.bat document.qmd
2. abnt.bat compila runner se target/release/runner.exe não existe.
3. runner é invocado com $PATH (diretorio do .qmd) + nome do arquivo.
4. runner resolve template_path:
   a. env var ABNNTEMPLATE
   b. fallback: <runner_dir>/../../templetes/template.tex
5. runner descobre .bib único no dir do template (se houver).
6. runner constrói args:
   [render, document.qmd,
    --metadata bibliography=<bib>,  (se .bib detectado)
    --template <template_path>]
7. runner executa `quarto render` com esses args.
8. Quarto → Pandoc → LuaLaTeX → PDF.
9. PDF gerado em <dirname(document.qmd)>/<basename(document.qmd)>.pdf.
```

## Application Flow (Renderização)

```
.qmd (input)
   ↓
Quarto CLI
   ↓
Pandoc (processa metadata + crossref)
   ↓
.tex (intermediário com template.tex)
   ↓
LuaLaTeX (compila .tex)
   ↓
PDF (output)
```

## APIs

### Rust (interno)

Não há API pública — apenas CLI.

### Pandoc Template (template.tex)

Variáveis reconhecidas:

| Variável | Tipo | Default |
|---|---|---|
| `$title$` | string | (Quarto default) |
| `$author$` | string | (Quarto default) |
| `$date$` | date | (Quarto default) |
| `$cidade$` | string | "São Paulo" |
| `$bibliography$` | path | (vazio, sem refs) |
| `$body$` | markdown | (Quarto default) |

### Pandoc Metadata (YAML do `.qmd`)

Mesmas variáveis acima + Quarto-specific:

| Variável | Efeito |
|---|---|
| `format: pdf: pdf-engine: lualatex` | Força LuaLaTeX |
| `format: pdf: keep-tex: true` | Mantém `.tex` intermediário |
| `format: pdf: toc: true` | Gera sumário |

## Persistence

Nenhuma. ABNT Engine é stateless — cada execução lê arquivos e gera PDF.

## Error Handling

| Erro | Causa | Tratamento |
|---|---|---|
| Quarto não instalado | Exit 127 | Mensagem clara + exit 1 |
| Template não encontrado | ABNT_TEMPLATE inválido + fallback ausente | Mensagem clara + exit 1 |
| Múltiplos `.bib` | Ambiguidade | Não injeta bibliography + warning |
| LaTeX erro de compilação | Erro sintático .tex | Exit code do LuaLaTeX propagado |
| PDF não gerado | Erro fatal LuaLaTeX | Exit code do LuaLaTeX propagado |

## Security

- **Command injection:** `Command::new("quarto")` em Rust (não shell).
- **Path traversal:** `Path::join` seguro (Rust stdlib).
- **Arbitrary code execution:** Pandoc-Crossref tem `--filter` flag — não usamos.
- **Supply chain:** Pinning de versões (Rust + Quarto + LuaLaTeX).

## Concurrency

- Single-threaded — cada `abnt.bat document.qmd` é uma execução isolada.
- Não há fila ou pool.

## External Integrations

- **Quarto** — CLI externa (chamada via `Command::new`).
- **Pandoc** — chamado por Quarto internamente.
- **LuaLaTeX** — chamado por Pandoc.
- **biblatex-abnt** — pacote LaTeX (CTAN).
- **abntex2 v1.9.7** — classe LaTeX (GitHub).

## Testing Strategy

Ver `docs/specs/testing-strategy.md`. Resumo:

- Camada 1 (cargo build/test) — ✅ automatizada, 13 testes.
- Camada 2 (smoke-test) — ⏳ script pronto, requer lualatex.
- Camada 3-6 — ⏳ manual, requer lualatex + inspeção visual.

## Deployment

N/A — ABNT Engine é CLI local, não há deployment.

## Operational Considerations

- **Quarto version:** recomendado Quarto >= 1.4 (Pandoc 3.x).
- **LuaLaTeX version:** TeX Live 2023+ ou MiKTeX 23+.
- **abntex2:** versão v1.9.7 (oficial, GitHub).
- **biblatex-abnt:** versão compatível com biblatex 3.19+.

## Known Limitations

1. Sem suporte a NBR 14724:2024 (regras mudaram; abntex2 não atualizado).
2. Sem suporte a Quadros via Pandoc-Crossref built-in.
3. Lista de ilustrações/tabelas sempre gerada (mesmo se vazia).
4. Compilação é lenta (~30s por documento típico) devido a biblatex + biber.
5. Windows-only para `abnt.bat`; cross-platform via `cargo run`.

## Future Evolution

| Evolução | Esforço | Benefício |
|---|---|---|
| Migração NBR 14724:2024 | Alto | Compliance atualizado |
| Suporte a Quadros | Médio | Tipos adicionais de float |
| CI/CD pipeline | Médio | Validação automática |
| VS Code tasks | Baixo | UX melhor |
| Documentação online | Médio | Discoverability |

## References

1. ADRs: `docs/decisions/00{1..5}-*.md`.
2. Estratégia de testes: `docs/specs/testing-strategy.md`.
3. Relatório de pesquisa: `docs/research/abnt-image-injection-research.md`.
4. Histórico: `CHANGELOG.md`.
