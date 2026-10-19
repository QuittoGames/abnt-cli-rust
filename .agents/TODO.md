# TODO

> Convenção: AGENTS.md §16 — seções `[H]` Human / `[A]` Agent / `[S]` Shared,
> IDs estáveis `TASK-00X`. `.agents/TASKS.md` permanece como lista de tarefas
> do subsistema de pesquisa (escopo diferente) — não duplica este arquivo.

## Escopo desta onda de trabalho

Variante escolar `templetes/template-canello-marques.tex` (spec aprovada pelo DEV).
Regras invioláveis:

1. `templetes/template.tex` permanece **inalterado**.
2. A variante é **cópia** do template-base com somente 3 classes de mudança:
   - bloco `capa` (linhas 245–254 da cópia) → capa escolar Canello Marques;
   - `\setlength{\parskip}{0pt}` (linha 138) → `6pt` (lista escrita da escola;
     divergência com o screenshot do Word deve ser **registrada**, nunca
     invertida silenciosamente);
   - comentários **diretamente ligados** a essas duas mudanças, marcando-os
     como overrides do Colégio Canello Marques.
3. Intocáveis: folha de rosto, sumário, bibliografia, paginação, seções,
   macros, pacotes, `runner/`, `abnt.bat`, testes do runner.
4. Nenhum metadado YAML novo (`title`, `author`, `date`, `cidade` já existem);
   `COLÉGIO CANELLO MARQUES` é constante da variante.

---

## Agent / Human / Shared

_Nenhuma task ativa — onda encerrada. Ver **Completed** (TASK-001..011)._

## Completed

- [x] [A] **TASK-012** Testes de coerência de manutenção da variante
  (`runner/tests/template_canello_marques.rs`, 3 testes integration/std):
  (a) variante reconstruível a partir do template-base — invariante
  anti-derivada: qualquer divergência fora das 2 regiões autorizadas
  quebra o teste; (b) contrato da capa escolar (constante, placeholders
  Pandoc, `\fontsize{20}{24}`/`\fontsize{16}{20}`, sem campos novos);
  (c) parskip 6pt na seção PARÁGRAFOS com `\tightlist` intacto.
  `cargo test --release`: **19 passed / 0 failed** (16 + 3).
- [x] [A] **TASK-008** RESOLVIDA (decisões DEV, 19/10/2026, chat):
  DATA/ANO → opção A, receita YAML `date: "2026-01-01"` +
  `date-format: "YYYY"` (validada: capa com `2026`; zero código).
  CITAÇÃO → ajuste autorizado com testes: `cite-method: biblatex` no
  YAML do `.qmd` (validado ponta-a-ponta: `\autocite`, seção REFERÊNCIAS
  com entrada resolvida pelo biber, zero mudança de runner/template)
  + testes de coerência entregues (TASK-012).
- [x] [A] **TASK-001** Pré-condições verificadas; correções de ambiente em
  user-space (repo intocado, sem sudo):
  `tlmgr --usermode --ignore-warning install babel-portuges` e
  `biblatex-abnt` (Fedora exige `--usermode`); fonte Times New Roman
  Bold/Italic/BoldItalic copiada de
  `~/Projects/scripts/.wintrasferlinux/Fonts/WindowsFonts/` para
  `~/.local/share/fonts/` + `fc-cache -f` + `luaotfload-tool --update`
  (antes só existia o Regular e `\bfseries` caía silenciosamente —
  afetava também o template-base).
- [x] [A] **TASK-009** Inspeção visual concluída (PNGs + `pdffonts`
  confirmando TimesNewRoman Bold embutido; capa 1 autor e 3 autores;
  página A4 única; sem rótulos didáticos; corpo com gap 6pt).
  Aprovação formal do DEV = TASK-010.
- [x] [A] **TASK-002** `templetes/template-canello-marques.tex` criada
  como cópia byte-a-byte; `git diff -- templetes/template.tex` vazio.
- [x] [A] **TASK-003** Bloco `capa` sobrescrito apenas na cópia
  (comentário de override Colégio Canello Marques; placeholders
  `$title$`/`$author$`/`$cidade$`/`$date$` preservados; sem metadados novos).
- [x] [A] **TASK-004** `parskip` 0pt → 6pt só na linha da seção
  PARÁGRAFOS, com comentário da divergência; `\tightlist` intacto
  (demais ocorrências continuam 0pt).
- [x] [A] **TASK-005** Diff base vs variante = somente capa + parskip +
  comentários; `git status` em `templetes/` = apenas arquivo novo;
  `runner/`, `abnt.bat`, `components/`, `refs.bib` intocados.
- [x] [A] **TASK-006** `cargo test --release` = 16 passed / 0 failed.
- [x] [A] **TASK-007** Render real com a variante exit 0 (PDF 4 págs A4
  + `.tex` com `COLÉGIO CANELLO MARQUES`, `parskip 6pt`, placeholders
  substituídos) e regressão com o template-base também exit 0.
