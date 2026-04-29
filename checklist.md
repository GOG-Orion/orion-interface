# Implementation Checklist

Este checklist organiza as proximas etapas de implementacao do Orion em fases curtas e verificaveis.

## 1. Governanca e catalogo

- [ ] Definir o modelo de catalogo com `upstreamRepo`, `orionRepo`, `releaseSource` e `status`.
- [ ] Classificar cada integracao como `upstream-original`, `orion-fork`, `orion-curated` ou `mixed`.
- [ ] Registrar autoria, licenca e fonte de release em cada integracao.
- [ ] Confirmar quais integracoes sao reais e quais ainda sao placeholders.

## 2. Repositorios e forks

- [ ] Confirmar quais repositorios ficam centralizados na organizacao Orion.
- [ ] Definir o upstream principal de cada fork Orion.
- [ ] Criar governanca de fork para cada repo curado.
- [ ] Garantir que melhorias de forks Orion possam voltar ao upstream via PR.

## 3. Sync e release

- [ ] Ajustar o workflow de sync para atualizar metadados sem publicar release automaticamente.
- [ ] Garantir que sync abra PR em vez de merge direto.
- [ ] Validar asset, digest e nome de release antes de promover qualquer integracao.
- [ ] Separar release upstream de release Orion curada no catalogo.

## 4. UI e install flow

- [ ] Manter o bloqueio de instalacao quando o cliente GOG estiver aberto.
- [ ] Refinar mensagens de bloqueio e de erro para o usuario.
- [ ] Confirmar que cada integracao listada na UI tem repo e asset validos.
- [ ] Melhorar a indicacao de origem da integracao na interface.

## 5. Seguranca e integridade

- [ ] Continuar validando SHA-256 quando o asset fornecer digest.
- [ ] Manter a rejeicao de scripts, executaveis e symlinks.
- [ ] Expandir regras por plataforma se necessario.
- [ ] Reforcar validacao de payload antes da copia final.

## 6. Documentacao e conduta

- [ ] Atualizar `research-update.md` sempre que uma decisao arquitetural mudar.
- [ ] Manter o `README.md` alinhado com a politica real do projeto.
- [ ] Garantir que `CODE_OF_CONDUCT.md` mencione credito, licenca e respeito aos autores originais.
- [ ] Garantir que `CONTRIBUTING.md` explique PRs, upstream e revisao humana.

## 7. Verificacao

- [ ] Rodar `pnpm typecheck`.
- [ ] Rodar `pnpm build`.
- [ ] Rodar `cargo fmt --check`.
- [ ] Rodar `cargo test` onde as dependencias do sistema estiverem instaladas.
- [ ] Revisar `git diff --check` antes de qualquer merge.
