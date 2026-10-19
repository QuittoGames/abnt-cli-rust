#!/bin/sh
# abnt — porte Linux (sh/zsh) do abnt.bat
# Wrapper do ABNT runner: orquestra compilacao e `quarto render`.
#
# Uso: abnt <filename.qmd>
#
# Funciona em qualquer POSIX sh (dash, bash, zsh). Suporte a symlink:
# se instalado em ~/.local/bin via symlink, `readlink -f` resolve de
# volta ao repositorio para encontrar runner/ e templetes/.

# ── Caminhos ──────────────────────────────────────────────────────────────────
SELF="$(readlink -f "$0" 2>/dev/null || echo "$0")"
ROOT="$(cd "$(dirname "$SELF")" && pwd)"
RUNNER_DIR="$ROOT/runner"
BIN="$RUNNER_DIR/target/release/runner"

# ── 1. Arg obrigatorio: filename ──────────────────────────────────────────────
if [ $# -lt 1 ] || [ -z "$1" ]; then
    echo "[ERROR] Uso: abnt <filename.qmd>"
    exit 1
fi

# ── 2. Binario nao existe? Compila ───────────────────────────────────────────
if [ ! -x "$BIN" ]; then
    echo "[INFO] Binario nao encontrado. Compilando..."

    if [ ! -d "$RUNNER_DIR" ]; then
        echo "[ERROR] Pasta 'runner' nao encontrada em: $ROOT"
        exit 1
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        echo "[ERROR] 'cargo' nao encontrado. Instale Rust: https://rustup.rs"
        exit 1
    fi

    if ! (cd "$RUNNER_DIR" && cargo build --release); then
        echo "[ERROR] Falha na compilacao. Verifique os erros acima."
        exit 1
    fi
fi

# ── 3. Checagem final pos-compilacao ─────────────────────────────────────────
if [ ! -x "$BIN" ]; then
    echo "[ERROR] Binario ainda ausente apos compilar. Verifique o Cargo.toml."
    exit 1
fi

# ── 4. Configurar TEXINPUTS para que LuaLaTeX encontre abnt-header.tex ────────
# Sintaxe kpathsea no Unix: separador ':', '//' = busca recursiva e o ':'
# final (elemento vazio) anexa aos paths default (nao os substitui).
# O runner tambem injeta TEXINPUTS internamente nas chamadas do quarto;
# aqui mantemos paridade com o abnt.bat.
TEMPLATE_DIR="$ROOT/templetes"
export TEXINPUTS="${TEMPLATE_DIR}//:${TEXINPUTS:-}"

# ── 5. Executa: PATH do sistema como arg1, filename como arg2 ─────────────────
# LD_LIBRARY_PATH: biber instalado em ~/.local/bin (binario do TeX Live tlnet)
# linka libcrypt.so.1 de ~/.local/lib (libxcrypt-compat, ausente no Fedora 44).
# O env propaga pelo runner -> quarto -> biber.
export LD_LIBRARY_PATH="$HOME/.local/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

# `exec` substitui o shell: exit code do runner propaga direto.
exec "$BIN" "$PATH" "$1"
