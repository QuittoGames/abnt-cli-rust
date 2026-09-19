# CONTEXT.md — Estado Atual da Pesquisa

> Estado local da pesquisa. Atualizado a cada milestone pelo
> `research-refactor-agent`. Representa **o que é**, não **o que deveria ser**.

## 1. Research Status

```yaml
research_status:
  active: "construção da arquitetura de agentes"
  completed: "8 agentes + 1 orquestrador + .agents/ estrutura"
  pending: "primeiro caso de uso real (validar com pesquisa de cônicas)"
```

## 2. Arquitetura Instalada

```yaml
agents_dir: "C:\\Users\\Quitto\\AppData\\Local\\hermes\\agents\\research\\"
agents_ativos:
  - research-refactor-agent (orquestrador)
  - literature-agent
  - web-research-agent
  - paper-research-agent
  - source-validator
  - evidence-agent
  - citation-agent
  - workflow-orchestrator
  - abnt-context-agent

registro_canonico: "C:\\Users\\Quitto\\AppData\\Local\\hermes\\agents-registry.md"
config: "C:\\Users\\Quitto\\AppData\\Local\\hermes\\config.yaml"
```

## 3. Projeto ABNT (Pipeline Alvo)

```yaml
abnt_engine:
  root: "D:\\Research\\ABNT"
  template: "D:\\Research\\ABNT\\templetes\\template.tex"
  header: "D:\\Research\\ABNT\\templetes\\abnt-header.tex"
  bib: "D:\\Research\\ABNT\\templetes\\refs.bib"
  runner: "D:\\Research\\ABNT\\runner\\target\\release\\runner.exe"
  launcher: "D:\\Research\\ABNT\\abnt.bat"
  decisions: "D:\\Research\\ABNT\\docs\\decisions\\"  # 5 ADRs
  specs: "D:\\Research\\ABNT\\docs\\specs\\"

projeto_pesquisa_alvo:
  root: "D:\\Research\\Works\\School\\Math\\Conicas_Matematica"
  index_qmd: "D:\\Research\\Works\\School\\Math\\Conicas_Matematica\\index.qmd"
  protegido: true
  excecao: "typos mecânicos"
```

## 4. Fontes

```yaml
sources:
  total: 0  # primeira pesquisa ainda não foi executada
  verified: 0
  high_quality: 0
  medium_quality: 0
  low_quality: 0
  low_confidence: 0
  rejected: 0
```

## 5. Evidências

```yaml
evidence:
  collected: 0
  validated: 0
  pending: 0
```

## 6. Lacunas (Gaps)

```yaml
gaps:
  - "Primeira execução real ainda não validada"
  - "Não há exemplo concreto de paper com claim + page number"
  - "Não há teste end-to-end com .qmd real do usuário"
```

## 7. Decisões

```yaml
decisions:
  - id: D001
    timestamp: 2026-09-02
    context: "Estrutura da arquitetura de agentes"
    proposal: "1 orquestrador + 8 sub-agentes especializados"
    decision: "Aceito"
    decided_by: "user (Quitto) — pedido inicial"
    consequences:
      - "8 personalidades a serem adicionadas ao config.yaml"
      - "registro a ser atualizado no agents-registry.md"

  - id: D002
    timestamp: 2026-09-02
    context: "Proteção do index.qmd"
    proposal: "READ permitido, WRITE proibido (exceto typos)"
    decision: "Aceito"
    decided_by: "user (Quitto) — protocolo SciRefactor"
    consequences:
      - "Agentes que tocam o index.qmd devem verificar antes de escrever"
      - "YAML header é permitido"

  - id: D003
    timestamp: 2026-09-02
    context: "Localização dos agentes"
    proposal: "agents/research/ no Hermes, .agents/ no projeto ABNT"
    decision: "Aceito"
    decided_by: "research-refactor-agent (proposta) + user (implícito)"
    consequences:
      - "Convenção: agentes globais em Hermes, estado do projeto em .agents/"
      - "agents-registry.md referencia agents/*/"
```

## 8. Consultas Realizadas

```yaml
queries:
  - ""
```

## 9. Estado do Workflow

```yaml
workflow:
  name: "Construção da arquitetura multi-agente"
  current_phase: "completion"
  parallel_groups_executed:
    - "[agent-creation] literature, source-validator, evidence, citation"
    - "[agent-creation] web-research, paper-research, workflow-orchestrator"
    - "[agent-creation] abnt-context-agent"
  parallel_groups_pending: []
  artifacts_produced:
    - "9 agent files em ~/.hermes/agents/research/"
    - "5 .agents/ files (AGENTS, RULES, REQUEST, CONTEXT, TASKS)"
    - "workflows/ directory"
    - "schemas/ directory"
  next_step: "update config.yaml + agents-registry.md"
```

## 10. Métricas

```yaml
metricas:
  agentes: 9
  arquivos_criados: ~17
  linhas_documentacao: ~5500
  cobertura_workflows: 1 workflow completo + 2 templates
  cobertura_schemas: 4 schemas YAML
  compatibilidade_abnt: "100% (todos agentes referenciam ABNT Engine)"
```

## 11. Próximo Passo

```yaml
proximo_passo:
  agent: "research-refactor-agent"
  acao: "atualizar config.yaml e agents-registry.md"
  data_alvo: 2026-09-02
  bloqueios: []
```

## 12. Handoff

```yaml
handoff:
  de: "research-refactor-agent (construção)"
  para: "user (Quitto) — revisão"
  entrega: "arquitetura completa + documentação + compatibilidade ABNT"
  pendente: "primeiro caso de uso real (pesquisa cônicas)"
```
