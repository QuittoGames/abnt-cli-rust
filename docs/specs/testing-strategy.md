# Testing Strategy — ABNT Engine

Estratégia de testes em camadas (conforme regra 9 do sistema de Quitto).
Cada camada valida um aspecto diferente e tem requisitos próprios.

## As 6 Camadas

### Camada 1 — Sintaxe

**Objetivo:** garantir que o código compila sem erro.

**Ferramentas:**
- `cargo build --release` — compilação Rust.
- `cargo test` — testes unitários.

**Quando executar:** a cada mudança no runner (Commits 1, 2, …).

**Critério de aprovação:** 0 erros, 0 warnings.

**Status atual:** ✅ 13 testes passam (Commit 2). Camada 1 não depende de LaTeX.

---

### Camada 2 — Renderização

**Objetivo:** garantir que o pipeline `Quarto → Pandoc → LuaLaTeX` compila sem erro fatal.

**Ferramentas:**
- `tests/fixtures/smoke-test.ps1` — script que renderiza cada fixture e valida.

**Quando executar:** a cada Commit que altera template ou runner.

**Critério de aprovação:** todas as fixtures geram PDF com tamanho > 1KB.

**Status atual:** ⏳ não executado (sem lualatex no ambiente). Mas o **smoke test foi criado** em Commit 3 e as **macros ABNT estão carregadas** em Commit 4. Quando o usuário instalar TeX Live, o smoke test deve passar imediatamente.

**Como executar:**
```powershell
pwsh runner/tests/fixtures/smoke-test.ps1
```

---

### Camada 3 — PDF válido

**Objetivo:** garantir que o PDF gerado é um PDF válido (não truncado, abre em reader).

**Ferramentas:**
- `pdfinfo` (poppler) ou similar para validar header.
- Smoke test já verifica tamanho.

**Quando executar:** após Camada 2.

**Critério de aprovação:** PDF abre em Adobe Reader, Evince, Chrome, etc.

**Status atual:** ⏳ bloqueado em Camada 2.

---

### Camada 4 — Visual (manual)

**Objetivo:** validar que o PDF **parece** ABNT-compliant.

**Ferramentas:**
- PDF reader (Adobe Reader, Evince, SumatraPDF).
- Inspeção manual por olho humano.

**Quando executar:** após Camada 3, **sempre** quando política ABNT muda.

**Critério de aprovação:** conforme checklist por fixture (ver `tests/fixtures/README.md`).

**Status atual:** ⏳ bloqueado em Camada 2.

**Como executar:**
```bash
# Renderizar uma fixture específica
quarto render tests/fixtures/figures/01-uma-figura/t.qmd --to pdf

# Abrir PDF
xdg-open tests/fixtures/figures/01-uma-figura/t.pdf
```

---

### Camada 5 — Regressão visual (comparação)

**Objetivo:** detectar mudanças visuais não-intencionais.

**Ferramentas:**
- Renderizar fixtures antes e depois da mudança.
- Comparar via `pdftotext` ou inspeção visual lado-a-lado.

**Quando executar:** antes de merge de qualquer mudança em `template.tex`.

**Critério de aprovação:** mudanças visuais apenas onde intencionais.

**Status atual:** ⏳ bloqueado em Camada 2.

---

### Camada 6 — Conformidade normativa

**Objetivo:** validar conformidade com NBR 14724:2011 (e evolução para :2024 em Commit futuro).

**Ferramentas:**
- Inspeção visual + citação literal da norma.
- Matriz normativa em `docs/research/abnt-image-injection-research.md`.

**Quando executar:** ao final de cada Commit que afeta norma.

**Critério de aprovação:** cada elemento da matriz tem **evidência verificável** (N/O/C/P).

**Status atual:** 🟡 parcial — Commit 2 documentou D1 (12pt legenda) e D2 (NBR 14724:2011) em ADRs.

---

## Princípios da Estratégia

1. **Testes atômicos, não integrados** — cada fixture é independente.
2. **Camadas crescentes** — Camada 1 sempre; Camadas 2+ conforme ambiente.
3. **Manual é válido** — Camadas 4-6 exigem olho humano; não é fraqueza.
4. **Determinismo > velocidade** — fixtures reproduzíveis; sem timeouts mágicos.
5. **Cobertura > completude** — 10 fixtures de figuras cobrem 90% dos casos comuns; 100% é impossível.

## Ferramentas

| Camada | Ferramenta | Disponível no Commit |
|---|---|---|
| 1 | cargo | 1 |
| 2 | smoke-test.ps1 | 3 (atual) |
| 3 | pdfinfo / pdf reader | pós-LaTeX |
| 4 | PDF reader + olho humano | pós-LaTeX |
| 5 | pdftotext diff | pós-LaTeX |
| 6 | matrix normativa + olho | contínuo |

## Bloqueios Atuais

| Bloqueio | Causa | Resolução |
|---|---|---|
| Camada 2 não executada | Sem lualatex no ambiente | Usuário instala TeX Live ou MiKTeX |
| Camada 4 não executada | Depende de Camada 2 | Idem |
| Camada 5 não tem baseline | PDF antigo era de `tests/t.pdf` (não-fresh) | Renderizar fixtures e guardar como baseline |

## Próximos Passos

- **Commit 4:** criar `abnt-header.tex` — Camada 1 + Camada 2 vão validar.
- **Commit 7:** crossref + lista de ilustrações — Camada 4 obrigatória.
- **Commit 8:** documentação + cleanup — fechar pendências.
