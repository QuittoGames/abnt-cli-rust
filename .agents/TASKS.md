# TASKS.md — Tarefas do Projeto de Pesquisa

> Lista de tarefas atuais, concluídas, bloqueadas e delegadas. Atualizado pelo
> `research-refactor-agent` a cada milestone.

## Legenda

```yaml
status:
  [TODO]       não iniciada
  [IN_PROGRESS] em execução
  [BLOCKED]    bloqueada (depende de decisão externa)
  [REVIEW]     aguardando revisão do usuário
  [COMPLETED]  concluída
  [CANCELLED]  cancelada

owner:
  [H]  Human (Quitto)
  [A]  Agent (orquestrador + sub-agentes)
  [S]  Shared (decisão compartilhada)

priority:
  [CRITICAL] [HIGH] [MEDIUM] [LOW]
```

## Em Progresso

- [ ] [A][HIGH] Atualizar `agents-registry.md` (registrar 8 novos agentes)
- [ ] [A][HIGH] Atualizar `config.yaml` (adicionar personalities)

## TODO

- [ ] [A][MEDIUM] Criar workflow exemplo `literature-review-conicas.md`
- [ ] [A][MEDIUM] Criar schemas YAML (source, evidence, decision, task)
- [ ] [S][HIGH] Validar arquitetura com primeiro caso de uso real

## Concluídas

### Agentes criados

- [x] [A] `research-refactor-agent.md` — orquestrador global
- [x] [A] `literature-agent.md` — descoberta papers acadêmicos
- [x] [A] `source-validator.md` — validação de qualidade
- [x] [A] `evidence-agent.md` — extração de evidências
- [x] [A] `citation-agent.md` — automação BibTeX
- [x] [A] `web-research-agent.md` — descoberta web
- [x] [A] `paper-research-agent.md` — análise profunda de papers
- [x] [A] `workflow-orchestrator.md` — coordenação executiva
- [x] [A] `abnt-context-agent.md` — contexto ABNT Engine

### Estrutura `.agents/`

- [x] [A] `.agents/AGENTS.md` — arquitetura de agentes do projeto
- [x] [A] `.agents/RULES.md` — regras invariáveis
- [x] [A] `.agents/REQUEST.md` — pedido original
- [x] [A] `.agents/CONTEXT.md` — estado da pesquisa
- [x] [A] `.agents/TASKS.md` — este arquivo
- [x] [A] `.agents/workflows/` — workflows paralelos
- [x] [A] `.agents/schemas/` — schemas YAML

## Bloqueadas

_Nenhuma tarefa bloqueada._

## USER_REVIEW

_Nenhuma decisão pendente do usuário no momento._

## Métricas

```yaml
agents_criados: 9 (1 orquestrador + 8 especializados)
workflows_documentados: 1+ (ver .agents/workflows/)
schemas_definidos: 4+ (ver .agents/schemas/)
total_linhas_documentacao: ~5000
data_conclusao_arquitetura: 2026-09-02
```
