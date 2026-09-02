# ABNT Engine

CLI wrapper Rust + Quarto + LuaLaTeX para renderizar `.qmd` em PDFs no padrão
ABNT brasileiro (NBR 14724:2011), usando a classe LaTeX [`abntex2`](https://github.com/abntex/abntex2).

---

## Quick Start

```bat
abnt.bat document.qmd
```

Pré-requisitos: Rust (cargo), Quarto, TeX Live (ou MiKTeX) com LuaLaTeX.

---

## Features

- Detecção automática de `template.tex` (env var `ABNT_TEMPLATE` > fallback relativo).
- Detecção automática de `.bib` único no diretório do template (cross-platform, Rust nativo).
- Injeção de `--metadata bibliography=...` para o Quarto.
- Integração Windows via `abnt.bat` (compila runner se necessário).
- Runner Rust cross-platform com exit codes corretos (não-zero em falha).
- Testes unitários para descoberta de arquivos e construção de args.
- Política ABNT separada em `abnt-header.tex` (NBR 14724:2011).
- Macros para figuras, tabelas, equações, listagens.
- Listas pré-textuais (ilustrações, tabelas, quadros).
- 13 fixtures de validação cobrindo todos os elementos.

---

## Decisões Arquiteturais

| Decisão | Status | ADR |
|---|---|---|
| BibLaTeX + biblatex-abnt (mantido) vs CSL (removido) | Aceito | [001](docs/decisions/001-bibliografia-biblatex-vs-csl.md) |
| NBR 14724:2011 + legenda 12pt / fonte 10pt | Aceito | [002](docs/decisions/002-norma-e-fonte-legenda.md) |
| Separação motor (template.tex) vs política (abnt-header.tex) | Aceito | [003](docs/decisions/003-separacao-motor-politica.md) |
| Listas pré-textuais geradas automaticamente | Aceito | [004](docs/decisions/004-listas-pre-textuais-automaticas.md) |
| Resultado final dos 8 commits | Aceito | [005](docs/decisions/005-resultado-final.md) |

Para a arquitetura completa, ver [`docs/specs/abnt-engine-architecture.md`](docs/specs/abnt-engine-architecture.md).
Para o histórico detalhado, ver [`CHANGELOG.md`](CHANGELOG.md).

---

## Macros ABNT Disponíveis (abnt-header.tex)

### Fonte de ilustração/tabela (NBR 14724:2011 §5.8 + 14724:2024 §5.8)

```latex
${ \fonteElaborado }$                       -> "Fonte: Elaborado pelo próprio autor."
${ \fonteAdaptado{LAMPORT (1994, p. 50)} }$ -> "Fonte: Adaptado de LAMPORT (1994, p. 50)."
${ \fonteRetirado{XYZ (ano)} }$             -> "Fonte: Retirado de XYZ (ano)."
${ \fonteTraduzido{XYZ (ano)} }$            -> "Fonte: Traduzido de XYZ (ano)."
```

### Listas pré-textuais

```latex
::: {.raw-latex}
\listadeilustracoes    % NBR 14724 §4.2.1.9 — lista de figuras
\listadetabelas        % NBR 14724 §4.2.1.8 — lista de tabelas
\listadequadros        % NBR 14724 §4.2.1.10 — lista de quadros (reservado)
:::
```

**Nota:** A partir de Commit 7, `\listadeilustracoes` e `\listadetabelas` são
geradas automaticamente no template após o sumário. Não é necessário
chamá-las manualmente.

### Tabela ABNT (NBR 14724 §5.9)

Pandoc-Crossref gera automaticamente caption "Tabela N – Título" com
formato ABNT (caption 12pt, fonte 10pt).

### Equações (NBR 14724 §5.10)

```markdown
$$
E = mc^2
$$ {#eq-einstein}
```

Numeração automática: (1), (2), ...

### Listagens (código)

```python
#| label: lst-exemplo
#| caption: "Exemplo de código"
print("Hello")
```

Pandoc-Crossref gera ambiente `figure` com caption "Listagem N".

---

## Project Structure

```
ABNT/
├─ abnt.bat                       # Windows launcher
├─ CHANGELOG.md                   # Histórico de mudanças (8 commits)
├─ README.md                      # Este arquivo
├─ docs/
│  ├─ decisions/                  # 5 ADRs (registro de decisões)
│  ├─ research/                   # Research reports
│  └─ specs/                      # Estratégia de testes + arquitetura
├─ runner/                        # Rust binary
│  ├─ src/
│  │  ├─ main.rs                  # CLI entry (cross-platform)
│  │  └─ utils/
│  │     └─ environment.rs        # Arg builder + .bib discovery (13 testes)
│  ├─ tests/
│  │  ├─ t.qmd                    # Example .qmd
│  │  ├─ fixtures/                # 13 fixtures .qmd + script smoke-test
│  │  │  ├─ figures/              # Figuras, tabelas, equações, etc.
│  │  │  ├─ references/refs.bib   # Fixture BibLaTeX
│  │  │  └─ smoke-test.ps1        # Camada 2 (renderização)
│  │  └─ t.pdf                    # Last rendered PDF
│  ├─ Cargo.toml
│  └─ target/
├─ templetes/
│  ├─ template.tex                # LaTeX template (motor estrutural)
│  └─ abnt-header.tex             # Política ABNT (NBR 14724:2011)
└─ Legacy/                        # Historical artifacts
   ├─ csl/                        # Moved here 2026-09-01 (orphan)
   ├─ abnt.bat
   ├─ inter.pdf
   └─ template.tex
```

---

## Requirements

### 1. Rust Toolchain
https://www.rust-lang.org/tools/install

### 2. Quarto
https://quarto.org/docs/get-started/

### 3. LaTeX Distribution (com LuaLaTeX)
- **TeX Live** (recomendado): https://www.tug.org/texlive/
- **MiKTeX**: https://miktex.org/download

---

## How it works

```
   .qmd ──┐
          │
   runner.exe ─→ detecta template (env var | relativo) ──┐
          │                                              │
          ├─ detecta .bib único no dir do template ──────┤
          │                                              │
          └─ constrói: quarto render <qmd>               │
                       --metadata bibliography=<file>    ├──→ Quarto → Pandoc → LuaLaTeX → PDF
                       --template <path>                │
                                                         ┘
```

### Ordem de resolução do template

1. **Env var `ABNT_TEMPLATE`** (mais específica, recomendado para setups fora de `templetes/`).
2. **Fallback relativo**: `<runner_dir>/../../templetes/template.tex`.
3. **Erro claro** se nenhum dos dois existir.

### Resolução de bibliografia

- Procura por **exatamente um arquivo `.bib`** no diretório do template.
- Se houver 0 ou >1 arquivos `.bib`, **não injeta** nada (decisão conservadora — evita ambiguidade silenciosa).
- O `.bib` detectado vira `--metadata bibliography=<filename>` para o Quarto.

---

## Usage

### Windows (recomendado)

```bat
abnt.bat document.qmd
```

O `.bat` automaticamente compila o runner se `target/release/runner.exe` não existir.

### Cross-platform (manual)

```bash
cd runner
cargo build --release
ABNT_TEMPLATE=/path/to/template.tex \
  ./target/release/runner "$PATH" document.qmd
```

### Variáveis de ambiente

| Variável | Descrição | Default |
|---|---|---|
| `ABNT_TEMPLATE` | Caminho completo para `template.tex` | `<runner_dir>/../../templetes/template.tex` |

### Variáveis dinâmicas (template)

Definidas no YAML do `.qmd` e processadas pelo Pandoc no `template.tex`.

| Variável | Descrição | Default |
|---|---|---|
| `cidade` | Cidade exibida na capa e folha de rosto | "São Paulo" (fallback) |
| `date` | Data exibida na capa e folha de rosto | (Quarto default: `today`) |
| `author` | Autor(es) do documento | (Quarto default: nenhum) |
| `title` | Título do documento | (Quarto default: nome do arquivo) |

**Exemplo:**

```yaml
---
title: "Meu Trabalho de Conclusão"
author: "Fulano de Tal"
date: 2026-09-01
cidade: "Rio de Janeiro"
---
```

---

## Tests

```bash
cd runner
cargo test --release
```

**13 testes unitários** cobrem:
- Descoberta de `.bib` (vazio, único, múltiplo, case-insensitive, com hífens, em dir inexistente).
- Resolução de template path (presente, ausente).
- Composição de args Quarto (`build`).
- Edge cases: diretórios `.bib`, arquivos não-`.bib`.

Para validação visual (Camadas 2-6), ver [`docs/specs/testing-strategy.md`](docs/specs/testing-strategy.md).

---

## Notas de Migração (a partir de versões antigas)

- O caminho `D:\Research\ABNT\runner` hardcoded em `abnt.bat` foi corrigido para usar `%~dp0\runner` (portabilidade).
- O caminho `D:\Research\ABNT\templetes\template.tex` hardcoded em `main.rs` foi substituído por resolução via env var + fallback.
- `cmd /C dir | findstr ".bib"` foi substituído por `std::fs::read_dir` (cross-platform, sem command injection).
- O arquivo CSL órfão (`csl/associacao-brasileira-de-normas-tecnicas.csl`) foi movido para `Legacy/`.

---

## License

MIT
