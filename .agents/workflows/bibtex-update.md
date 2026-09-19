# Workflow: Atualização Incremental de BibTeX

> Workflow focado em adicionar novas fontes ao `refs.bib` com validação
> rigorosa. Mais simples que revisão de literatura completa — útil quando
> usuário já tem pesquisa e quer apenas expandir referências.

## 1. Objetivo

Adicionar 1-N novas fontes ao `D:\Research\ABNT\templetes\refs.bib` com
validação completa.

## 2. Tasks

```yaml
T001_resolve_metadata:
  agent: literature-agent  # ou manual
  acao: "para cada nova fonte, resolver metadata (DOI, autor, ano, venue)"

T002_validate_quality:
  agent: source-validator
  acao: "aplicar hierarquia de evidência (mínimo MEDIUM para adicionar)"

T003_check_duplicates:
  agent: citation-agent
  acao: "verificar se já existe no refs.bib (DOI, título, autor+ano)"

T004_generate_entry:
  agent: citation-agent
  acao: "gerar BibTeX entry com cite_key único"

T005_show_diff:
  agent: citation-agent
  acao: "apresentar diff antes de aplicar"

T006_apply_diff:
  agent: citation-agent
  acao: "aplicar via patch (não write_file)"

T007_validate_bib:
  agent: citation-agent
  acao: "rodar biber --tool ou similar; detectar duplicatas finais"
```

## 3. Regras

```yaml
fontes_aceitas:
  - HIGH: "sempre adicionadas"
  - MEDIUM: "adicionadas com ressalva no CONTEXT.md"
  - LOW_CONFIDENCE: "NÃO adicionadas sem autorização do usuário"
  - REJECTED: "nunca adicionadas"

duplicatas:
  - DOI_exato: "reportar, não adicionar"
  - titulo_similar_95pct: "investigar (mesma fonte, metadata diferente?)"
  - autor_ano_similares: "investigar (preprint vs published?)"
```

## 4. Saída Esperada

```yaml
arquivos_modificados:
  - "D:\\Research\\ABNT\\templetes\\refs.bib"

arquivos_atualizados:
  - "D:\\Research\\ABNT\\.agents\\CONTEXT.md"
  - "D:\\Research\\ABNT\\.agents\\TASKS.md"

relatorio:
  - "total_added: N"
  - "rejected_duplicates: M"
  - "rejected_invalid: K"
  - "needs_user_review: J"
```

## 5. Compilação Pós-Update

```bash
# Após refs.bib atualizado, compilar para validar
cd D:\Research\ABNT
abnt.bat runner\tests\t.qmd
```

Se a compilação **falhar**:
- Reverter o patch
- Investigar erro de sintaxe BibLaTeX
- Reportar ao usuário
