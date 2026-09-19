# AGENT_ARCHITECTURE.md — Arquitetura de Agentes do Projeto de Pesquisa (ABNT)

> Este arquivo define a arquitetura de agentes, responsabilidades e
> coordenação para projetos de pesquisa científica que usam o **ABNT Engine**
> (`D:\Research\ABNT\`).
>
> **Nota:** o nome foi alterado de `AGENTS.md` para `AGENT_ARCHITECTURE.md`
> para evitar conflito com a convenção de "AGENTS.md" usada em outros
> frameworks (que tem regras de proteção próprias).

## 1. Identidade do Projeto

```yaml
project_type: scientific_research_with_abnt_pipeline
engine: ABNT Engine (Quarto + Pandoc + LuaLaTeX + abntex2 + biblatex-abnt)
norm: NBR 14724:2011
document_protected: <project_root>/index.qmd
bibliography_path: D:\Research\ABNT\templetes\refs.bib
```

## 2. Arquitetura Multi-Agente

### 2.1 Camadas

```text
┌─────────────────────────────────────────────────────────────────┐
│  Camada 0 — Orquestrador Global                                  │
│  research-refactor-agent                                         │
│  (delega, paraleliza, consolida, reporta)                        │
└─────────────────────────────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌────────────────┐  ┌────────────────────┐  ┌─────────────────┐
│ Camada 1 —     │  │ Camada 1 —         │  │ Camada 1 —      │
│ Contexto       │  │ Descoberta         │  │ Coordenação     │
│ abnt-context-  │  │ literature-agent   │  │ workflow-       │
│ agent          │  │ web-research-agent │  │ orchestrator    │
│ (snapshot ABNT)│  │ paper-research-    │  │ (grafo de tasks)│
│                │  │ agent              │  │                 │
└────────────────┘  └────────────────────┘  └─────────────────┘
        │                       │
        ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  Camada 2 — Processamento                                        │
│  source-validator → evidence-agent → citation-agent              │
│  (valida → extrai → BibTeX)                                      │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│  Camada 3 — Estado (.agents/)                                    │
│  CONTEXT.md (fontes/evidências)                                  │
│  TASKS.md (workflow atual)                                       │
│  REQUEST.md (pedido original)                                   │
│  RULES.md (regras invariáveis)                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Sub-agentes Registrados (carregar sob demanda)

| Agente | Categoria | Modo | Quando carregar |
|---|---|---|---|
| `research-refactor-agent` | orquestrador | `all` | **SEMPRE** quando há pedido de pesquisa |
| `abnt-context-agent` | contexto | `subagent` | **SEMPRE** se projeto usa ABNT Engine |
| `literature-agent` | descoberta | `subagent` | quando precisa de papers acadêmicos |
| `web-research-agent` | descoberta | `subagent` | quando precisa de docs/web/vendors |
| `paper-research-agent` | descoberta | `subagent` | quando precisa analisar paper em profundidade |
| `source-validator` | processamento | `subagent` | após qualquer descoberta, antes de aceitar evidência |
| `evidence-agent` | processamento | `subagent` | após validação, para extrair claims |
| `citation-agent` | processamento | `subagent` | após validação, para atualizar `.bib` |
| `workflow-orchestrator` | coordenação | `subagent` | quando ≥ 5 tasks com dependências cruzadas |

## 3. Handoffs

### Entrada (quem chama a pesquisa):
- `user (Quitto)` — pedido direto
- `doc-writer` — precisa de pesquisa para documentar
- `teacher` — precisa de pesquisa para ensinar
- `architecture-analyzer` — pesquisa de padrões
- `refactoring-engineer` — pesquisa de best practices

### Saída (pra quem a pesquisa entrega):
- `user (Quitto)` — relatório final + decisões pendentes
- `doc-writer` — pesquisa consolidada para documentação
- `abnt-context-agent` — quando precisa revalidar restrições do pipeline
- `citation-agent` — quando uma fonte foi aceita (entra na fila de BibTeX)

## 4. Restrições Invioláveis do Projeto

```yaml
nunca_modificar_sem_autorizacao:
  - "<project>/index.qmd"   # documento científico (exceto typos)
  - "D:\\Research\\ABNT\\templetes\\template.tex"
  - "D:\\Research\\ABNT\\templetes\\abnt-header.tex"
  - "D:\\Research\\ABNT\\runner\\"

modificaveis_sem_autorizacao:
  - "<project>/.agents/*"   # estado da pesquisa
  - "<project>/docs/*"     # research, decisions, specs, diagrams
  - "D:\\Research\\ABNT\\templetes\\refs.bib"  # via citation-agent validado
  - "<project>/index.qmd" YAML header apenas  # autor, date, cidade, format
```

## 5. MCPs e Skills

### 5.1 MCPs Priorizados (para pesquisa)

```yaml
pesquisa_academica: arxiv
docs_oficiais:      microsoft_learn, context7, twilio_docs, vercel
busca_generica:     web_search, web_extract
conhecimento:       wolfram (quando útil)

evitar:
  - duplicar capacidade entre MCPs
  - chamar MCP não-relevante ao domínio
```

### 5.2 Skills Priorizadas

```yaml
metodologia:    scientific-research, project-research
descoberta:     arxiv, grounded-citations, blogwatcher
orquestracao:   graph-engineering, task-scheduler
```

## 6. Versionamento

```yaml
agents_version: 1.0.0
data: 2026-09-02
autor: Quitto
status: production-ready

historico:
  - "1.0.0 (2026-09-02) — arquitetura inicial: 8 agentes + 1 orquestrador"
```

## 7. References

- `D:\Research\ABNT\docs\specs\abnt-engine-architecture.md`
- `D:\Research\ABNT\docs\decisions\` (5 ADRs)
- `D:\Research\ABNT\docs\research\abnt-image-injection-research.md`
- `D:\Research\ABNT\templetes\template.tex`
- `D:\Research\ABNT\templetes\abnt-header.tex`
- `~/.hermes/agents/research/research-refactor-agent.md`
- `~/.hermes/agents/agents-registry.md`
