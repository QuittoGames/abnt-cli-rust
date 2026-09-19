# REQUEST.md — Pedido Original

> Este arquivo contém o **pedido bruto original** do usuário. Não deve ser
> reinterpretado silenciosamente. O orquestrador (`research-refactor-agent`)
> deve preservar a intenção original e apenas **decompor** em subtasks, não
> **modificar o objetivo**.

## Pedido Original

```yaml
data: 2026-09-02
solicitante: Quitto
canal: research-refactor-agent (ativação inicial)
objetivo_central: >
  Criar um agente global de pesquisa científica (research-refactor-agent)
  para o Hermes, que melhore a arquitetura de agentes, workflows, MCPs
  e skills de pesquisa do ecossistema, com integração específica ao
  ABNT Engine (Quarto + Pandoc + LuaLaTeX + abntex2 + biblatex-abnt).

contexto_tecnico:
  abnt_engine: "D:\\Research\\ABNT"
  template_tex: "D:\\Research\\ABNT\\templetes\\template.tex"
  abnt_header_tex: "D:\\Research\\ABNT\\templetes\\abnt-header.tex"
  refs_bib: "D:\\Research\\ABNT\\templetes\\refs.bib"
  document_protegido: "D:\\Research\\Works\\School\\Math\\Conicas_Matematica\\index.qmd"
  hermes_dir: "C:\\Users\\Quitto\\AppData\\Local\\hermes"

restricoes_explicitas:
  - "Não modificar conteúdo científico do index.qmd (apenas YAML/typos)"
  - "Não inventar fontes/DOIs/papers"
  - "Preservar rigor científico"
  - "Manter rastreabilidade de evidências"
  - "Não duplicar capacidades entre MCPs/skills"

criterios_sucesso:
  - "Agente global research-refactor-agent criado e registrado"
  - "Sub-agentes especializados criados (≥ 5)"
  - "Estrutura .agents/ completa (AGENTS.md, RULES.md, TASKS.md, CONTEXT.md, REQUEST.md)"
  - "Workflows paralelos documentados"
  - "Schemas YAML criados"
  - "config.yaml e agents-registry.md atualizados"
  - "Compatibilidade verificada com ABNT Engine"
```

## Decomposição Inicial (pelo research-refactor-agent)

```yaml
tasks_principais:
  - "Criar research-refactor-agent.md (agente global orquestrador)"
  - "Criar literature-agent.md (descoberta acadêmica)"
  - "Criar source-validator.md (validação de qualidade)"
  - "Criar evidence-agent.md (extração de evidências)"
  - "Criar citation-agent.md (automação BibTeX)"
  - "Criar web-research-agent.md (descoberta web)"
  - "Criar paper-research-agent.md (análise profunda de papers)"
  - "Criar workflow-orchestrator.md (orquestração executiva)"
  - "Criar abnt-context-agent.md (contexto ABNT Engine)"
  - "Criar estrutura .agents/ (AGENTS.md, RULES.md, TASKS.md, CONTEXT.md, REQUEST.md)"
  - "Criar workflows em .agents/workflows/"
  - "Criar schemas em .agents/schemas/"
  - "Atualizar config.yaml e agents-registry.md"

tasks_completadas:
  - "research-refactor-agent.md"  # status: completed
  - "literature-agent.md"  # status: completed
  - "source-validator.md"  # status: completed
  - "evidence-agent.md"  # status: completed
  - "citation-agent.md"  # status: completed
  - "web-research-agent.md"  # status: completed
  - "paper-research-agent.md"  # status: completed
  - "workflow-orchestrator.md"  # status: completed
  - "abnt-context-agent.md"  # status: completed
```

## Estado Atual

> Atualizado pelo `research-refactor-agent` em cada milestone.

```yaml
fase: "execução"
data_inicio: 2026-09-02
progresso_pct: 92  # após criar .agents/ e workflows
proximo_passo: "atualizar config.yaml e agents-registry.md"
```
