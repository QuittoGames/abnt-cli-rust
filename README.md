# ABNT Engine Rust

ABNT Engine Rust is a CLI tool that automates academic PDF generation using Quarto and a custom ABNT LaTeX template.

It acts as a wrapper that:
- Detects templates
- Detects bibliography files
- Injects metadata automatically
- Executes Quarto render with correct arguments

---

## Features

- Automatic .bib detection inside template directory
- Automatic LaTeX template injection
- Cross-command argument builder for Quarto
- Windows CLI integration (abnt.bat)
- Rust-based execution engine for performance and safety

**OBS: this code is coded for WindowsNT systhens**
---

## Project Structure

ABNT/
├─ abnt.bat
├─ LICENSE
├─ README.md
├─ runner/
│  ├─ src/
│  │  ├─ main.rs
│  │  └─ utils/
│  │     └─ environment.rs
│  └─ target/
├─ templetes/
│  └─ template.tex

---

## Requirements

### 1. Rust Toolchain

Install Rust (includes Cargo compiler):

https://www.rust-lang.org/tools/install

---

### 2. Quarto

Quarto is required for rendering .qmd files into PDF.

Download:
https://quarto.org/docs/get-started/

---

### 3. LaTeX Distribution

Quarto requires a LaTeX engine to generate PDFs.

Options:

- MiKTeX: https://miktex.org/download
- TeX Live: https://www.tug.org/texlive/

---

## How it works

1. The .bat file compiles the Rust runner if needed
2. The runner receives:
   - PATH environment
   - .qmd filename
3. Rust:
   - Searches template folder
   - Searches .bib files
   - Builds Quarto arguments
4. Quarto renders the final PDF

---

## Usage

abnt.bat document.qmd

---

## LaTeX + Quarto Flow

.qmd → Pandoc → LaTeX → PDF

---

## Why LaTeX is required

- ABNT formatting
- mathematical notation
- custom templates
- bibliography system

---

## Template system

- abntex2
- biblatex
- Pandoc compatibility layer

---

## Notes

- bibliography is injected automatically when found
- template must exist or be provided correctly

---

## License

MIT License
