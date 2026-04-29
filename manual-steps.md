# Manual Steps

Este arquivo lista as decisoes humanas e os passos manuais que precisam ser feitos antes ou junto da automacao no ecossistema GOG-Orion.

## 1. Decisoes que precisam ser tomadas manualmente

### 1.1 Classificacao de cada integracao

Para cada integracao, decidir uma destas opcoes:

- `upstream-original`
- `orion-fork`
- `orion-curated`
- `mixed`

Decidir tambem:

- qual e o `upstreamRepo` oficial;
- se existe `orionRepo`;
- qual e a fonte real de release;
- se a integracao e mantida apenas como catalogo referenciado ou como fork local.

### 1.2 Politica de autoria e licenca

Confirmar manualmente:

- qual licenca se aplica a cada repo ou fork;
- se o fork Orion pode publicar release propria;
- como o credito aos autores originais sera exibido;
- se alguma integracao precisa de aviso adicional de autoria ou redistribuicao;
- se a licenca permite modificacoes, redistribuicao e releases curadas.

### 1.3 Politica de sustentacao

Definir:

- se o projeto vai aceitar doacoes;
- quais canais de doacao serao usados;
- se doacoes sustentam apenas infraestrutura e manutencao;
- se alguma funcao ou repositorio tera acesso restrito ou nao.

### 1.4 Escopo do catalogo

Decidir se o catalogo vai listar:

- apenas forks Orion;
- apenas originais;
- ambos, com filtragem por origem;
- releases curadas e releases upstream separadamente.

## 2. Token de acesso e automacao

### 2.1 Quando nao precisa de token

Em geral, nao e necessario criar token se a automacao:

- apenas le repositorios publicos;
- atualiza arquivos no proprio repo local;
- roda validacoes sem publicar nada fora do repositorio atual.

### 2.2 Quando e melhor criar um token

Considere criar um token de acesso quando a automacao precisar:

- abrir PRs automaticamente;
- criar branches de sync automatizadas;
- atualizar repositorios fora do fluxo normal;
- acessar repositorios privados;
- reduzir risco de rate limit da API do GitHub;
- operar workflows em mais de um repositorio da organizacao.

### 2.3 Opcao recomendada de token

Preferir nesta ordem:

1. GitHub App, se o objetivo for automacao mais segura e controlada;
2. Fine-grained personal access token, se a App nao for viavel;
3. Classic PAT, somente se o fluxo realmente exigir e nao houver alternativa melhor.

### 2.4 Permissoes que podem ser necessarias

Dependendo do fluxo, o token pode precisar de:

- leitura de conteudo;
- leitura de metadata de release;
- escrita em branches do repo da organizacao;
- abertura de pull requests;
- leitura de actions;
- gerenciamento de releases, se a publicacao for automatizada.

### 2.5 O que nunca automatizar sem revisao

Nao automatizar sem revisao humana:

- publicar release de fork Orion;
- sobrescrever tags existentes;
- trocar a fonte oficial de uma integracao;
- aceitar um upstream novo como fonte sem validar autoria e licenca;
- mover integracoes entre `orion-fork` e `upstream-original` sem aprovacao.

## 3. Passos manuais por area

### 3.1 Organizacao e governanca

- Confirmar quais repositorios ficam sob a organizacao Orion.
- Definir o que e fork, o que e original e o que e mirror curado.
- Registrar o upstream principal de cada fork.
- Escrever o aviso de autoria e licenca no README e nos forks.
- Revisar o Code of Conduct para incluir credito, licenca e respeito ao trabalho original.

### 3.2 Repositorios de integracao

- Confirmar se cada integracao existe de fato.
- Decidir se o repo do Orion tera release propria ou apenas mirror.
- Atualizar `manifest.json` e metadados de release apenas depois da validacao.
- Abrir PRs para o upstream original quando houver melhoria util.

### 3.3 Sync e releases

- Confirmar quais repositorios entram no workflow de sync.
- Definir se o sync apenas atualiza metadados ou tambem prepara PR.
- Revisar manualmente qualquer mudanca de `releaseSource`.
- Conferir asset, digest e nome do release antes de publicar ou promover.

### 3.4 UI e install flow

- Confirmar quais integracoes aparecem na UI.
- Confirmar quais repositorios estao prontos para instalacao.
- Validar a mensagem de bloqueio quando o GOG estiver aberto.
- Revisar se cada integracao tem um asset e um source validos.

### 3.5 Documentacao

- Revisar README apos qualquer mudanca de arquitetura.
- Manter `research-update.md` apenas como nota de pesquisa e direcionamento.
- Expandir docs sempre que um fluxo novo for definido.

## 4. Checklist de execucao sugerida

- [ ] Definir a classificacao de todas as integracoes conhecidas.
- [ ] Confirmar o upstream principal de cada fork Orion.
- [ ] Decidir se a organizacao vai usar GitHub App ou PAT.
- [ ] Criar o token ou a App com as permissoes corretas, se necessario.
- [ ] Registrar a politica de credito, licenca e doacao no Code of Conduct.
- [ ] Finalizar o catalogo com `upstreamRepo`, `orionRepo`, `releaseSource` e `status`.
- [ ] Revisar o workflow de sync para abrir PR e nao publicar release automaticamente.
- [ ] Confirmar quais integracoes aparecem na UI.
- [ ] Validar manualmente as fontes de release e os digests antes de promover uma integracao.
- [ ] Atualizar README e docs quando a politica de origem mudar.

## 5. Regra pratica

Se uma tarefa:

- altera autoria;
- troca a fonte de release;
- publica release;
- acessa repositorio privado;
- ou faz push em outro repo da organizacao;

ela deve ser revisada manualmente antes de entrar em producao.
