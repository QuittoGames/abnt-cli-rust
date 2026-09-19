# RULES.md — Regras Invariáveis do Projeto de Pesquisa

> Regras que **NUNCA** podem ser quebradas sem autorização explícita do
> usuário. Aplicam-se a **todos** os agentes do subsistema de pesquisa.

## 1. Proteção do Documento Científico

```yaml
regra: "index.qmd é PROTEGIDO"
leitura: permitida
escrita: PROIBIDA por padrão

excecao_unica: "fix typos mecânicos"
  tipos_permitidos:
    - erro ortográfico
    - erro de digitação
    - acentuação faltando
  tipos_proibidos:
    - reescrever frase
    - alterar estrutura
    - modificar argumentação
    - mudar significado
    - substituir palavras por preferência

quando_em_duvida: INTERROMPER, REPORTAR, AGUARDAR AUTORIZAÇÃO
```

## 2. Template e Política ABNT

```yaml
template_tex:
  path: "D:\\Research\\ABNT\\templetes\\template.tex"
  modificacao: PROIBIDA sem ADR novo + autorização

abnt_header_tex:
  path: "D:\\Research\\ABNT\\templetes\\abnt-header.tex"
  modificacao: PROIBIDA sem ADR novo + autorização

runner_rust:
  path: "D:\\Research\\ABNT\\runner\\"
  modificacao: delegar para refactoring-engineer (não fazer in-place)
```

## 3. Integridade Científica

```yaml
nunca:
  - inventar fonte, DOI, paper, citação, autor, ano
  - apresentar inferência como fato confirmado
  - usar LLM como fonte científica primária
  - apagar pesquisa válida sem justificativa
  - sobrescrever referências sem validação
  - modificar conteúdo científico do index.qmd sem autorização
  - fabricar saída quando subagente falhou
```

## 4. BibTeX

```yaml
arquivo: "D:\\Research\\ABNT\\templetes\\refs.bib"
atualizacao: SOMENTE via citation-agent validado

validacoes_obrigatorias:
  - DOI resolve (HTTP 200 ou Crossref API)
  - metadados completos (autor, título, ano, venue)
  - sem duplicatas
  - sem chaves duplicadas
  - sintaxe BibLaTeX válida

fontes_LOW_CONFIDENCE: NAO adicionar sem autorização do usuário
fontes_REJECTED: NAO adicionar em nenhuma circunstância
```

## 5. Evidências

```yaml
cada_evidencia_deve_ter:
  - claim (afirmação)
  - source (fonte)
  - location (página/seção/tabela/figura) — não inventar
  - confidence (HIGH/MEDIUM/LOW)
  - limitations (quando aplicável)
```

## 6. Paralelismo

```yaml
regra: "Delegar em paralelo quando possível"
criterio: "Se subT_A não depende de subT_B, executar em paralelo"
excecao: "Dependências reais (precisa do output de B para A)"

limite_padrao: 3 subagentes paralelos
  configuravel_via: max_concurrent_children
```

## 7. Hierarquia de Evidência

```yaml
nivel_1: documentação oficial     → peso máximo
nivel_2: standards (RFC, ABNT)    → peso máximo
nivel_3: peer-reviewed            → peso alto
nivel_4: repositórios oficiais    → peso alto
nivel_5: vendor docs              → peso médio-alto
nivel_6: código real              → peso médio
nivel_7: maintainer blog          → peso médio
nivel_8: blog especialista        → peso baixo-médio
nivel_9: comunidade               → peso mínimo
nivel_10: não revisado            → peso zero (NÃO usar como evidência)

decisao_critica: exigir ≥ 2 fontes de nível 1-3
```

## 8. Estado da Pesquisa

```yaml
CONTEXT.md: deve refletir estado ATUAL (não histórico completo)
TASKS.md: deve refletir tarefas REAIS (não plano ideal)
REQUEST.md: pedido ORIGINAL (não reinterpretar silenciosamente)
RULES.md: regras INVARIÁVEIS (mexer só com autorização)
```

## 9. Autoridade do Usuário

```yaml
decisao_arquitetural: SEMPRE do usuário (agente = PROPOSAL)
decisao_cientifica: SEMPRE do usuário
autorizacao_especial:
  - modificar index.qmd conteúdo
  - adicionar ADR novo
  - modificar template.tex
  - modificar abnt-header.tex
  - aceitar fonte LOW_CONFIDENCE
```

## 10. Honesto Brutal

```yaml
quando_nao_souber: "Não encontrei evidência suficiente"
quando_evidencia_fraca: "Fonte de nível X — limitada"
quando_contradição: "Fontes A e B divergem — contexto: ..."
quando_gap: "Não cobrimos o aspecto Y — sugestão: ..."
quando_falha_subagente: "Subagente X falhou — não fabrico resultado"
```

## 11. Cleanup

```yaml
arquivos_temporarios:
  localizacao: "%TEMP%\\research\\" (Windows) / "/tmp/research/" (Linux)
 何时_remover: imediatamente após uso
  excecao: persistência apenas com autorização

downloads_papers:
  permitido: apenas para papers HIGH/MEDIUM com relevância direta
  proibido: para papers LOW_CONFIDENCE ou apenas por completude
```

## 12. Rastreabilidade

```yaml
toda_decisao_arquitetural: registrada em .agents/decisions/ ou docs/decisions/
toda_evidencia: tem source_id + location
toda_task: tem id único + status + owner
toda_fonte: tem citation_key no refs.bib (após validação)
```

## 13. Versionamento

```yaml
agents_version: 1.0.0
data_inicio: 2026-09-02
regras_invariaveis: 13
agentes_ativos: 8 + 1 orquestrador
```

## 14. Anti-Patterns

```yaml
evitar:
  - criar agente para tarefa one-shot (usar agente existente)
  - duplicar capacidade entre MCPs
  - duplicar skill existente
  - inferir resultado quando subagente falhou
  - classificar fonte como HIGH sem peer-review verificado
  - gerar BibTeX para fonte não validada
  - atualizar CONTEXT.md com "deveria" em vez de "está"
  - usar medium/blog pessoal como evidência primária
```
