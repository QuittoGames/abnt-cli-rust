# ADR 004 — Listas Pré-Textuais Automáticas no Template

**Status:** Aceito
**Data:** 2026-09-01
**Contexto:** ABNT Engine — Commit 7

## Context

Após Commit 4, o `abnt-header.tex` definiu macros `\listadeilustracoes`
e `\listadetabelas` para gerar listas pré-textuais. Mas essas macros
**só eram geradas se chamadas manualmente** pelo usuário em raw LaTeX.

Isso causava:

- ❌ Documentos sem ABNT-compliance (sem listas pré-textuais).
- ❌ Cada usuário tinha que lembrar de chamar as macros.
- ❌ Documentos acadêmicos reais (TCCs, dissertações, teses) sempre têm
  essas listas — não é opt-in.

## Decision

**Gerar automaticamente** as listas pré-textuais no `template.tex`
imediatamente após o sumário, **antes** do conteúdo textual:

```latex
\tableofcontents*
\cleardoublepage
\listadeilustracoes    % NBR 14724 §4.2.1.9
\cleardoublepage
\listadetabelas        % NBR 14724 §4.2.1.8
\cleardoublepage
\textual
```

**Comportamento:**

- Listas são **sempre geradas**, mesmo se vazias.
- Se não houver figuras/tabelas no documento, a lista fica vazia mas
  o capítulo pré-textual aparece (com título ABNT).
- Usuário pode **opt-out** comentando as linhas no `template.tex`.

## Consequences

### Positivas

- ✅ ABNT-compliance out-of-the-box — usuário não precisa lembrar.
- ✅ Padrão ABNT NBR 14724:2011 §4.2 seguido sem configuração.
- ✅ Documentos acadêmicos completos por default.

### Negativas

- ❌ Listas vazias geram página "sem conteúdo" se não houver figuras.
- ❌ Usuários que querem opt-out precisam editar `template.tex`.

### Mitigações

- Para documentos sem figuras, a lista vazia é aceitável (página
  pré-textual com apenas o título).
- Opt-out documentado no comentário do template.

## Validation

- ✅ Commit 7 criado com sucesso.
- ✅ Camada 1 ainda passa (13 testes Rust).
- ⏳ Camada 2 (renderização) requer lualatex.

## References

1. NBR 14724:2011 §4.2 (elementos pré-textuais).
2. NBR 14724:2011 §4.2.1.9 (lista de ilustrações).
3. NBR 14724:2011 §4.2.1.8 (lista de tabelas).
4. abntex2 v1.9.7 — suporte nativo a `\listoffigures` e `\listoftables`.
