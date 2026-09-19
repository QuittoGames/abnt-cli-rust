# Workflow: Revisão de Literatura — Exemplo Cônicas

> Workflow de exemplo mostrando como `research-refactor-agent` orquestra uma
> pesquisa real. Este é o template canônico para revisão de literatura no
> ABNT Engine.

## 1. Objetivo

Mapear o estado da arte em **seções cônicas** para o TCC de matemática
(`D:\Research\Works\School\Math\Conicas_Matematica\index.qmd`).

## 2. Estratégia Geral

```yaml
modo: parallel_with_dependencies
total_subtasks: 8
parallel_groups: 4
critical_path: [T001, T003, T006, T008]
estimated_time: "60-90 min"
```

## 3. Task Graph

```text
              ┌─ T002 (web-research) ─┐
T001 (lit) ──┤                       ├── T003 (validate) ──┬─ T004 (analyze P001) ──┐
              └─ T005 (analyze P003) ─┘                     ├─ T005 (analyze P003) ──┤
                                                              │                       ├── T006 (evidence) ──┬─ T007 (bibtex) ──┬─ T008 (consolidate)
                                                              └─ ... outros papers ───┘                        └─────────────────┘
```

### 3.1 Tasks

```yaml
T001_literature_discovery:
  agent: literature-agent
  parallel_group: 1
  status: pending
  depends_on: []
  input:
    query: "conic sections"
    period: "2010-2025"
    target_count: 20
  output: ".agents/research/T001-candidates.yaml"
  acceptance_criteria:
    - "≥15 papers retornados"
    - "metadata completa (DOI/URL/autor/ano)"
    - "preliminary_relevance atribuída"

T002_web_discovery:
  agent: web-research-agent
  parallel_group: 1
  status: pending
  depends_on: []
  input:
    query: "conic sections ellipse hyperbola parabola official docs"
    target_tiers: [1, 2, 3, 5]  # docs, standards, institucionais, vendors
  output: ".agents/research/T002-web-sources.yaml"

T003_validate_sources:
  agent: source-validator
  parallel_group: 2
  status: pending
  depends_on: [T001, T002]
  input:
    sources: "T001 ∪ T002"
  output: ".agents/research/T003-validated.yaml"
  acceptance_criteria:
    - "cada fonte classificada (HIGH/MEDIUM/LOW/LOW_CONFIDENCE)"
    - "LOW_CONFIDENCE flagada para revisão do usuário"

T004_analyze_paper_1:
  agent: paper-research-agent
  parallel_group: 3
  status: pending
  depends_on: [T003]
  input:
    paper_id: "P_HIGEST_1"  # mais relevante do T003
  output: ".agents/research/T004-P001-profile.yaml"

T005_analyze_paper_2:
  agent: paper-research-agent
  parallel_group: 3
  status: pending
  depends_on: [T003]
  input:
    paper_id: "P_HIGEST_2"
  output: ".agents/research/T005-P002-profile.yaml"

T006_extract_evidence:
  agent: evidence-agent
  parallel_group: 4
  status: pending
  depends_on: [T003, T004, T005]
  input:
    profiles: [T004, T005]
    objective: "claim sobre cônicas para o TCC"
  output: ".agents/research/T006-evidence-table.yaml"
  acceptance_criteria:
    - "toda evidência tem page/section"
    - "matriz cross-source construída"
    - "gaps identificados"

T007_generate_bibtex:
  agent: citation-agent
  parallel_group: 4
  status: pending
  depends_on: [T003]
  input:
    validated_sources: "T003 (apenas HIGH/MEDIUM)"
  output: ".agents/research/T007-bibtex.diff"
  acceptance_criteria:
    - "diff mostrado antes de aplicar"
    - "sem duplicatas"
    - "sintaxe BibLaTeX válida"

T008_consolidate_report:
  agent: research-refactor-agent
  parallel_group: 5
  status: pending
  depends_on: [T001, T002, T003, T006, T007]
  input:
    all_artifacts: "T001-T007"
  output: ".agents/research/T008-final-report.md"
  acceptance_criteria:
    - "executive summary"
    - "tabela de evidências"
    - "gaps listados"
    - "decisões pendentes para o usuário"
    - "BibTeX diff confirmado"
```

### 3.2 Execution Plan

```yaml
rodada_1 (paralelo):
  - T001
  - T002

rodada_2:
  - T003 (após T001 + T002)

rodada_3 (paralelo):
  - T004
  - T005
  # + T007 (não depende de T004/T005)

rodada_4:
  - T006 (após T004 + T005)

rodada_5 (final):
  - T008 (após T001, T002, T003, T006, T007)
```

## 4. Handoffs

```yaml
T001 → T003: "lista de papers candidatos"
T002 → T003: "lista de fontes web"
T003 → T004, T005, T007: "fontes validadas"
T004, T005 → T006: "perfis de papers + claims"
T006, T007 → T008: "evidências + BibTeX diff"
T008 → user: "relatório final consolidado"
```

## 5. Critérios de Sucesso

```yaml
sucesso_quando:
  - "≥10 papers HIGH/MEDIUM coletados"
  - "≥5 evidências com page number"
  - "BibTeX atualizado com sources validadas"
  - "Cross-validation ≥ 2 fontes por claim principal"
  - "Gaps honestos listados"
  - "Decisões pendentes explicitadas"
  - "index.qmd NÃO modificado"

falha_quando:
  - "qualquer fonte inventada (DOI, autor, ano)"
  - "BibTeX gravado sem validação"
  - "evidência sem page/section"
  - "index.qmd modificado indevidamente"
```

## 6. Recuperação Após Interrupção

```yaml
resume_apos_interrupcao:
  acao: "ler .agents/graph/tasks.json"
  identificar:
    - tasks com status "in_progress" órfãs
    - tasks "pending" prontas para executar
  retomar: "a partir do último estado válido"
```

## 7. Anti-Patterns a Evitar

```yaml
evitar:
  - usar arXiv + Google Scholar para a mesma query (duplicar)
  - validar fontes LOW_CONFIDENCE como HIGH
  - aceitar paper sem relevance_score > 0.3
  - usar blog pessoal como evidência primária
  - gravar BibTeX sem diff
  - modificar index.qmd
  - reportar como completo se T006 ou T007 falhou
```
