# ADR 002 — Norma ABNT de Referência e Tamanho da Fonte da Legenda

**Status:** Aceito
**Data:** 2026-09-01
**Contexto:** ABNT Engine — definição de norma de referência para implementação

## Context

Duas decisões normativas precisavam ser tomadas antes de implementar política ABNT para figuras:

1. **Qual versão da NBR 14724 seguir como referência?**
2. **Qual tamanho de fonte usar nas legendas de figuras?**

Estas decisões afetam TODO o sistema de formatação ABNT do engine.

## Decision

### D1 — NBR 14724:2011 (compatível com abntex2 oficial)

**Decisão:** Implementar conforme **NBR 14724:2011**, que é a versão oficialmente suportada pela classe `abntex2` (classe LaTeX brasileira para ABNT).

**Justificativa primária:**
- abntex2 oficial (v1.9.7, última release) declara explicitamente compatibilidade com:
  - NBR 14724:2011
  - NBR 10520:2002
  - NBR 6023:2002
  - NBR 6024:2012
- Mudar para NBR 14724:2024 exigiria modificar abntex2 (fork ou patch) — fora do escopo deste projeto.

**Consequência:** futuras atualizações do engine que migrem para NBR 14724:2024 são aceitas, mas não são responsabilidade imediata.

### D2 — Legenda no mesmo tamanho do corpo (12pt), fonte/notas menores (10pt)

**Decisão:** 
- **Caption (legenda)** = 12pt (mesmo tamanho do corpo) — interpretação abntex2 oficial.
- **Fonte da ilustração** = 10pt (menor).
- **Notas** = 10pt (menor).

**Justificativa primária:**

> abntex2 v1.8 changelog (2013/08/26, oficial):
> "changed the font size of name of tables and figures due to a **congruent interpretation of ABNT NBR 14724:2011 and IBGE rules**: **the font size of the name of tables and figures must be the same of the text**. The font size of sources and notes are keep the same: a smaller and uniform one."

**Conflito com manuais USP/UFRGS:** alguns manuais acadêmicos sugerem legenda em 10pt, mas o abntex2 oficial diverge. Como o projeto usa abntex2, **seguimos a interpretação oficial abntex2** até que surja fonte primária conclusiva.

**Marcação UNVERIFIED:** Se você (Quitto) tiver acesso ao texto literal da NBR 14724:2024 §5.1 que defina explicitamente o tamanho da legenda, posso revisar esta decisão.

## Consequences

### Positivas

- ✅ Compatibilidade imediata com abntex2 — sem necessidade de patch.
- ✅ Interpretação oficial alinhada com a maioria das instituições brasileiras que usam abntex2.
- ✅ Diferenciação clara: caption = destaque visual do conteúdo; fonte/notas = informação auxiliar menor.

### Negativas

- ❌ Alguns manuais acadêmicos podem divergir (USP/UFRGS) — não há conflito direto se o usuário ajustar manualmente.
- ❌ NBR 14724:2024 tem mudanças não capturadas (ex.: "Elaborado pelo próprio autor" vs "Fonte: o autor").

### Mitigações

- A política ABNT ficará em `templetes/abnt-header.tex` (a ser criado em Commit 4), facilmente editável.
- Decisões de override via YAML do Quarto serão permitidas (ex.: `caption.fontsize: small`).

## Validation

- ✅ Decisão alinhada com abntex2 changelog (fonte primária).
- ⏳ Validação visual será feita em Commit 4 com fixtures.

## References

1. abntex2 GitHub: https://github.com/abntex/abntex2/blob/master/doc/latex/abntex2/README
2. abntex2 v1.8 (2013/08/26): "changed the font size of name of tables and figures..."
3. NBR 14724:2011 (compatibilidade declarada por abntex2).
4. abntex2 v1.9.6 (2016/02/26): caption separator = endash conforme NBR 14724:2011.
