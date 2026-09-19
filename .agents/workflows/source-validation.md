# Workflow: Validação de Fonte Única (Investigação)

> Workflow rápido para quando o usuário pede validação de uma fonte
> específica (ex: "essa referência serve?", "is this paper peer-reviewed?").

## 1. Objetivo

Aplicar hierarquia de evidência a uma única fonte e produzir classificação
justificada.

## 2. Tasks

```yaml
T001_resolve_metadata:
  agent: citation-agent (legado) ou manual
  input: "DOI/URL/título da fonte"
  output: "metadata completa (autor, ano, venue, abstract)"

T002_check_venue:
  agent: source-validator
  acao: "verificar se venue é peer-reviewed, indexado em Scopus/WoS/SciELO"

T003_check_author:
  agent: source-validator
  acao: "verificar track record do autor no campo"

T004_check_atualidade:
  agent: source-validator
  acao: "verificar ano de publicação vs. estado da arte"

T005_classify:
  agent: source-validator
  acao: "atribuir HIGH/MEDIUM/LOW/LOW_CONFIDENCE/REJECTED"

T006_justify:
  agent: source-validator
  acao: "produzir justificativa explícita + red flags + strengths"
```

## 3. Saída

```yaml
classificacao:
  fonte: "..."
  evidence_hierarchy_level: 1-10
  classificacao_final: "HIGH | MEDIUM | LOW | LOW_CONFIDENCE | REJECTED"
  justificativa: "..."
  red_flags: [...]
  strengths: [...]
  sugestao: "usar como evidência primária | complementar | substituir | descartar"
```

## 4. Decisão de Uso

```yaml
se_HIGH:
  acao: "adicionar ao pipeline (literature → evidence → bibtex)"
se_MEDIUM:
  acao: "adicionar com nota de limitação no CONTEXT.md"
se_LOW_CONFIDENCE:
  acao: "submeter ao usuário — não usar sem aprovação"
se_REJECTED:
  acao: "não usar; reportar motivo"
```
