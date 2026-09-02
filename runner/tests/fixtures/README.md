# Test Fixtures — ABNT Engine

Fixtures .qmd para validação visual do pipeline ABNT Engine. Cada fixture é
autocontida em seu diretório e renderiza um PDF que deve ser inspecionado
manualmente em PDF reader.

## Estrutura

```
tests/fixtures/
├── figures/
│   ├── images/                      # imagens placeholder compartilhadas
│   │   ├── placeholder.png          # 400x300
│   │   ├── placeholder-medium.png   # 600x450
│   │   ├── placeholder-small.png    # 200x150
│   │   └── placeholder-large.png   # 1200x900
│   ├── 01-uma-figura/               # 1 figura simples
│   ├── 02-multiplas-figuras/        # 5 figuras
│   ├── 03-referencia-cruzada/       # @fig-label
│   ├── 04-lista-de-ilustracoes/     # 3 figuras + \listoffigures
│   ├── 05-imagem-propria/           # Fonte: Elaborado pelo próprio autor
│   ├── 06-imagem-adaptada/          # Fonte: Adaptado de AUTOR (ano)
│   ├── 07-subfiguras/               # subfigures com subcaption
│   ├── 08-imagem-grande/            # > largura da página
│   ├── 09-imagem-pequena/           # 3cm
│   ├── 10-sem-imagens/              # sanity check (sem figuras)
│   ├── 11-cidade-customizada/       # override da variável cidade
│   ├── 12-tabela-abnt/              # tabela com \IBGEtab + \tabelaIBGE
│   └── 13-equacao-e-listagem/       # equação + listagem de código
├── references/
│   └── refs.bib                     # fixture .bib com 3 entradas
└── tables/                          # [a ser criado em Commit futuro]
```

## Como Executar

### Smoke test (Camada 2 — renderização)

```bash
cd runner
bash tests/fixtures/smoke-test.sh
```

O script percorre cada fixture, renderiza via `abnt.bat`, e valida que
o PDF foi gerado com tamanho > 1KB.

### Validação visual manual (Camada 4)

```bash
cd runner
# Renderizar uma fixture específica
ABNT_TEMPLATE=/path/to/template.tex \
  ./target/release/runner "$PATH" \
  tests/fixtures/figures/01-uma-figura/t.qmd

# Abrir PDF em PDF reader
xdg-open tests/fixtures/figures/01-uma-figura/t.pdf
```

### Comparação antes/depois (Camada 5 — regressão)

```bash
# Antes de mudança
for f in tests/fixtures/figures/*/t.qmd; do
  quarto render "$f" --to pdf
done

# Aplicar mudança em template

# Depois
for f in tests/fixtures/figures/*/t.qmd; do
  quarto render "$f" --to pdf
done

# Diff visual: comparar PDFs com pdftotext ou similar
```

## Status por Fixture

| # | Fixture | Commit 1-3 | Após Commit 4 (abnt-header) | Após Commit 6 |
|---|---|---|---|---|
| 01 | uma-figura | ⏳ sem ABNT caption | ✅ "Figura 1 – Título" | ✅ |
| 02 | multiplas-figuras | ⏳ numeração básica | ✅ numeração contínua 1-5 | ✅ |
| 03 | referencia-cruzada | ⏳ crossref básico | ✅ @fig-label hiperlink | ✅ |
| 04 | lista-de-ilustracoes | ⏳ sem lista | ✅ \listadeilustracoes | ✅ |
| 05 | imagem-propria | ⏳ fonte manual | ✅ \fonteElaborado | ✅ |
| 06 | imagem-adaptada | ⏳ fonte manual | ✅ \fonteAdaptado | ✅ |
| 07 | subfiguras | ⏳ sem subcaption | ✅ (a)(b)(c) com subcaption | ✅ |
| 08 | imagem-grande | ⏳ escala padrão | ✅ escala para \linewidth | ✅ |
| 09 | imagem-pequena | ⏳ alinhamento padrão | ✅ centralizado | ✅ |
| 10 | sem-imagens | ✅ sanity check | ✅ compila sem figuras | ✅ |
| 11 | cidade-customizada | n/a | n/a | ✅ |
| 12 | tabela-abnt | n/a | n/a | ✅ "Tabela 1 – Título" + IBGEtab |
| 13 | equacao-e-listagem | n/a | n/a | ✅ numeração automática |

## Macros ABNT Disponíveis (Commit 4+)

Após Commit 4, o `abnt-header.tex` carrega o seguinte conjunto de macros:

### Fonte da ilustração (NBR 14724:2011 §5.8)

| Macro | Uso | Saída |
|---|---|---|
| `${ \fonteElaborado }$` | Fonte do próprio autor | "Fonte: Elaborado pelo próprio autor." |
| `${ \fonteAdaptado{XYZ (ano)} }$` | Fonte adaptada | "Fonte: Adaptado de XYZ (ano)." |
| `${ \fonteRetirado{XYZ (ano)} }$` | Fonte retirada | "Fonte: Retirado de XYZ (ano)." |
| `${ \fonteTraduzido{XYZ (ano)} }$` | Fonte traduzida | "Fonte: Traduzido de XYZ (ano)." |

**Sintaxe Pandoc:** usar inline raw LaTeX via `$ ... $`.

### Lista de ilustrações (NBR 14724:2011 §4.2.1.9)

```markdown
::: {.raw-latex}
\listadeilustracoes
:::
```

Gera capítulo "LISTA DE ILUSTRAÇÕES" com índice de figuras.

**Nota:** A partir de Commit 7, este comando é gerado **automaticamente** no
template após o sumário. A chamada manual só é necessária se o usuário
quiser a lista **dentro do conteúdo textual** (não pré-textual).

### Subfiguras

Subfiguras funcionam via `subcaption` (carregado automaticamente). Use:

```markdown
::: {#fig-grupo layout-ncol=3}
![A](a.png){#fig-a}
![B](b.png){#fig-b}
![C](c.png){#fig-c}
Grupo de subfiguras.
:::
```

### Lista de tabelas (NBR 14724:2011 §4.2.1.8)

```markdown
::: {.raw-latex}
\listadetabelas
:::
```

Gera capítulo "LISTA DE TABELAS" com índice de tabelas.

### Lista de quadros (NBR 14724:2011 §4.2.1.10)

```markdown
::: {.raw-latex}
\listadequadros
:::
```

⚠️ Pandoc-Crossref não gera índice de quadros automaticamente; este
comando é reservado para extensão futura.

### Tabela ABNT (NBR 14724:2011 §5.9)

Para tabelas geradas por Pandoc-Crossref, usar syntax Markdown:

```markdown
| Col A | Col B |
|-------|-------|
| A1    | B1    |

: Título da tabela {#tbl-label}

Fonte: Elaborado pelo próprio autor.
```

Para tabelas com mais controle (formato IBGE), usar macro raw LaTeX:

```latex
\begin{table}[htb]
  \centering
  \caption{Título da tabela}
  \IBGEtabfontsize
  \begin{tabular}{cc}
    \toprule
    Col A & Col B \\
    \midrule
    A1 & B1 \\
    \bottomrule
  \end{tabular}
  \fonte{Fonte da tabela.}
\end{table}
```

### Equações (NBR 14724:2011 §5.10)

```markdown
$$
E = mc^2
$$ {#eq-einstein}
```

Numeração automática: (1), (2), ... Crossref via `@eq-label`.

### Listagens (código) — sem regra ABNT específica

Bloco de código com label gera caption automática:

````markdown
```{python}
#| label: lst-exemplo
#| caption: "Exemplo de código"
print("Hello")
```
````

Pandoc-Crossref gera ambiente `figure` para o bloco.

## Validação Normativa por Fixture

| Fixture | Norma ABNT | Verificação |
|---|---|---|
| 01-uma-figura | NBR 14724:2011 §5.8 | Caption acima da imagem, formato "Figura 1 – Título" |
| 02-multiplas-figuras | NBR 14724:2011 §5.8 | Numeração contínua (1, 2, 3, 4, 5) |
| 03-referencia-cruzada | NBR 6024:2012 | Crossref funcional |
| 04-lista-de-ilustracoes | NBR 14724:2011 §4.2.1.9 | Lista pré-textual "LISTA DE ILUSTRAÇÕES" |
| 05-imagem-propria | NBR 14724:2024 §5.8 | "Elaborado pelo próprio autor" |
| 06-imagem-adaptada | NBR 14724:2024 §5.8 + NBR 10520:2023 | "Adaptado de AUTOR (ano, p. X)" |
| 07-subfiguras | Convenção acadêmica | (a), (b), (c) sublegendas |
| 08-imagem-grande | NBR 14724:2011 §5.8 | Não overflow |
| 09-imagem-pequena | NBR 14724:2011 §5.8 | Centralizada |
| 10-sem-imagens | NBR 14724:2011 §5.8 | Compila sem erro |

## Critérios de Aprovação

Para cada fixture, o PDF deve:

1. **Compilar sem erro** (Camada 2).
2. **Ter tamanho > 1KB** (Camada 3).
3. **Renderizar figura visível** (Camada 4).
4. **Caption ABNT-compliant** (Camada 4 — após Commit 4).
5. **Crossref funcional** se aplicável (Camada 4).
6. **Lista de ilustrações** se aplicável (Camada 4 — após Commit 4).
7. **Conformidade visual** com NBR 14724 (Camada 6 — manual).
