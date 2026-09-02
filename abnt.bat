@echo off
setlocal enabledelayedexpansion

:: ── Caminhos ──────────────────────────────────────────────────────────────────
set "ROOT=%~dp0"
set "RUNNER_DIR=%ROOT%runner"
set "BIN=%RUNNER_DIR%\target\release\runner.exe"

:: ── 1. Arg obrigatório: filename ──────────────────────────────────────────────
if "%~1"=="" (
    echo [ERROR] Uso: abnt.bat ^<filename.qmd^>
    exit /b 1
)

:: ── 2. Binário não existe? Compila ───────────────────────────────────────────
if not exist "%BIN%" (
    echo [INFO] Binario nao encontrado. Compilando...

    if not exist "%RUNNER_DIR%" (
        echo [ERROR] Pasta 'runner' nao encontrada em: %ROOT%
        exit /b 1
    )

    where cargo >nul 2>&1
    if errorlevel 1 (
        echo [ERROR] 'cargo' nao encontrado. Instale Rust: https://rustup.rs
        exit /b 1
    )

    pushd "%RUNNER_DIR%"
    cargo build --release
    set "BUILD_ERR=!errorlevel!"
    popd

    if !BUILD_ERR! neq 0 (
        echo [ERROR] Falha na compilacao. Verifique os erros acima.
        exit /b 1
    )
)

:: ── 3. Checagem final pós-compilação ─────────────────────────────────────────
if not exist "%BIN%" (
    echo [ERROR] Binario ainda ausente apos compilar. Verifique o Cargo.toml.
    exit /b 1
)

:: ── 4. Configurar TEXINPUTS para que LuaLaTeX encontre abnt-header.tex ────────
:: O template.tex usa \input{abnt-header} que é resolvido em runtime.
:: Como LuaLaTeX compila em diretório temporário do Quarto, precisamos
:: adicionar o diretório do template (que contém abnt-header.tex) ao TEXINPUTS.
set "TEMPLATE_DIR=%ROOT%templetes"
set "TEXINPUTS=%TEMPLATE_DIR%;%TEXINPUTS%"

:: ── 5. Executa: PATH do sistema como arg1, filename como arg2 ─────────────────
"%BIN%" "%PATH%" "%~1"
