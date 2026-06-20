@echo off
setlocal

REM =========================
REM CONFIG
REM =========================
set "TEMPLATE=D:\Research\ABNT\template.tex"
set "BIB=referencias.bib"

REM =========================
REM CHECK PANDOC
REM =========================
where pandoc >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERRO] Pandoc nao encontrado no PATH
    exit /b 1
)

REM =========================
REM INPUT
REM =========================
if "%~1"=="" (
    echo [ERRO] Uso: abnt arquivo.md
    exit /b 1
)

set "INPUT=%~1"

if not exist "%INPUT%" (
    echo [ERRO] Arquivo nao encontrado: %INPUT%
    exit /b 1
)

set "NAME=%~n1"
set "TEX=%NAME%.tex"
set "OUTPUT=%NAME%.pdf"

REM =========================
REM BUILD
REM =========================
echo.
echo =========================
echo Compilando %INPUT%
echo =========================

pandoc "%INPUT%" ^
  --template="%TEMPLATE%" ^
  --top-level-division=chapter ^
  --bibliography="%BIB%" ^
  -o "%TEX%"

if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha Markdown -> LaTeX
    exit /b 1
)

REM =========================
REM COMPILACAO LATEX (SEM LATEXMK)
REM =========================
xelatex -interaction=nonstopmode "%TEX%"
xelatex -interaction=nonstopmode "%TEX%"

if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha LaTeX
    exit /b 1
)

echo.
echo [SUCESSO] PDF gerado: %OUTPUT%

endlocal
pause@echo off
setlocal

REM =========================
REM CONFIG
REM =========================
set "TEMPLATE=D:\Research\ABNT\template.tex"
set "BIB=referencias.bib"

REM =========================
REM CHECK PANDOC
REM =========================
where pandoc >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo [ERRO] Pandoc nao encontrado no PATH
    exit /b 1
)

REM =========================
REM INPUT
REM =========================
if "%~1"=="" (
    echo [ERRO] Uso: abnt arquivo.md
    exit /b 1
)

set "INPUT=%~1"

if not exist "%INPUT%" (
    echo [ERRO] Arquivo nao encontrado: %INPUT%
    exit /b 1
)

set "NAME=%~n1"
set "TEX=%NAME%.tex"
set "OUTPUT=%NAME%.pdf"

REM =========================
REM BUILD
REM =========================
echo.
echo =========================
echo Compilando %INPUT%
echo =========================

pandoc "%INPUT%" ^
  --template="%TEMPLATE%" ^
  --top-level-division=chapter ^
  --bibliography="%BIB%" ^
  -o "%TEX%"

if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha Markdown -> LaTeX
    exit /b 1
)

REM =========================
REM COMPILACAO LATEX (SEM LATEXMK)
REM =========================
xelatex -interaction=nonstopmode "%TEX%"
xelatex -interaction=nonstopmode "%TEX%"

if %ERRORLEVEL% neq 0 (
    echo [ERRO] Falha LaTeX
    exit /b 1
)

echo.
echo [SUCESSO] PDF gerado: %OUTPUT%

endlocal
pause