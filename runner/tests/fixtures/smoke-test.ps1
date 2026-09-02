# smoke-test.ps1
# Camada 2 de testes — renderização.
# Renderiza cada fixture via Quarto e valida que o PDF é gerado.
#
# Pré-requisitos: quarto, lualatex, abntex2, biblatex, biber.
#
# Uso:
#   pwsh tests/fixtures/smoke-test.ps1
#
# Exit codes:
#   0 = todas as fixtures renderizam
#   1 = pelo menos uma falhou

param(
    [string]$Template = "D:\Research\ABNT\templetes\template.tex",
    [string]$QuartoPath = "quarto"
)

$ErrorActionPreference = "Stop"
$fixturesRoot = Join-Path $PSScriptRoot "."
$pass = 0
$fail = 0
$failedFixtures = @()

Write-Host "=== ABNT Engine — Smoke Test (Camada 2) ===" -ForegroundColor Cyan
Write-Host "Template: $Template"
Write-Host "Quarto:   $QuartoPath"
Write-Host ""

# Verificar pré-requisitos
Write-Host "Verificando pré-requisitos..."
try {
    $null = & $QuartoPath --version 2>&1
    Write-Host "  [OK] quarto detectado" -ForegroundColor Green
} catch {
    Write-Host "  [ERRO] quarto não encontrado: $QuartoPath" -ForegroundColor Red
    exit 2
}

if (-not (Test-Path $Template)) {
    Write-Host "  [ERRO] template não existe: $Template" -ForegroundColor Red
    exit 2
}
Write-Host "  [OK] template existe" -ForegroundColor Green
Write-Host ""

# Iterar sobre cada fixture
$fixtures = Get-ChildItem -Path $fixturesRoot -Recurse -Filter "t.qmd" | Sort-Object FullName

foreach ($qmd in $fixtures) {
    $dir = $qmd.DirectoryName
    $name = $qmd.BaseName
    $rel = $qmd.FullName.Substring($fixturesRoot.Length)

    Write-Host "→ $rel" -ForegroundColor Yellow

    Push-Location $dir
    try {
        # Limpar PDFs antigos
        Get-ChildItem -Filter "*.pdf" -ErrorAction SilentlyContinue | Remove-Item -Force

        # Tentar renderizar
        $env:ABNT_TEMPLATE = $Template
        $output = & $QuartoPath render $qmd.Name --to pdf 2>&1 | Out-String

        # Verificar PDF gerado
        $pdf = Join-Path $dir "$name.pdf"
        if (Test-Path $pdf) {
            $size = (Get-Item $pdf).Length
            if ($size -gt 1024) {
                Write-Host "  [OK] PDF gerado: $size bytes" -ForegroundColor Green
                $pass++
            } else {
                Write-Host "  [FALHA] PDF muito pequeno: $size bytes" -ForegroundColor Red
                $fail++
                $failedFixtures += $rel
            }
        } else {
            Write-Host "  [FALHA] PDF não foi gerado" -ForegroundColor Red
            Write-Host "  Output: $output" -ForegroundColor DarkGray
            $fail++
            $failedFixtures += $rel
        }
    } catch {
        Write-Host "  [FALHA] Exceção: $_" -ForegroundColor Red
        $fail++
        $failedFixtures += $rel
    } finally {
        Pop-Location
    }
    Write-Host ""
}

# Resumo
Write-Host "=== Resumo ===" -ForegroundColor Cyan
Write-Host "Total:  $($fixtures.Count)"
Write-Host "Pass:   $pass" -ForegroundColor Green
Write-Host "Fail:   $fail" -ForegroundColor $(if ($fail -eq 0) { "Green" } else { "Red" })

if ($fail -gt 0) {
    Write-Host ""
    Write-Host "Fixtures que falharam:" -ForegroundColor Red
    foreach ($f in $failedFixtures) {
        Write-Host "  - $f" -ForegroundColor Red
    }
    exit 1
}

exit 0
