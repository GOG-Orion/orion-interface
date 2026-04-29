Foquei no ecossistema **GOG-Orion**, porque ele e o conjunto de repositorios ligado ao objetivo de baixar, manter e curar integracoes do GOG Galaxy:

* `GOG-Orion/.github`
* `GOG-Orion/orion-interface`
* `GOG-Orion/galaxy-integration-steam`

## 1. Leitura atual do projeto

O objetivo do Orion ainda faz sentido: organizar integracoes comunitarias do GOG Galaxy 2.0, reduzir friccao para o usuario final e evitar que cada integracao fique espalhada por ZIPs, forks e instrucoes diferentes.

O ponto importante que ficou mais claro agora e este:

* o Orion nao deve virar apenas um "catalogo central" mantido dentro do repositorio base/original;
* o catalogo precisa apontar para repositorios reais por integracao;
* em alguns casos isso sera um fork Orion;
* em outros, sera o repositório original, quando ainda nao houver fork curado;
* o repositorio base pode guardar governanca, docs, policies e o mapa do ecossistema, mas nao deveria concentrar toda a manutencao de cada integracao.

Isso resolve a tensao entre centralizacao e origem: o usuario ve um catalogo unico, mas a manutencao continua distribuida entre upstream original, fork Orion e release curada.

## 2. Estado atual que ja foi resolvido

### `GOG-Orion/orion-interface`

O estado atual do `orion-interface` ja nao bate com boa parte dos problemas antigos do rascunho inicial.

Ja esta resolvido:

* o app React 18 usa `react-dom/client`;
* as abas de Source Code, Contributors e Configurations existem de verdade;
* a versao da UI esta alinhada com a versao de pacote;
* a verificacao de release nao depende mais de uma string hardcoded no frontend;
* a comparacao de versao agora usa `semver`;
* a instalacao nao depende mais de `install.bat`;
* o fluxo de instalacao bloqueia quando o cliente GOG esta aberto;
* o install root pode ser descoberto e tambem sobrescrito;
* checksum SHA-256 e validacao de payload ja fazem parte do fluxo;
* docs basicas, policy docs e sync workflow ja foram criados.

Ou seja: o problema principal nao e mais "fazer o app funcionar do zero". O que falta agora e consolidar o modelo de catalogo, procedencia e distribuicao.

### `GOG-Orion/.github`

O repositorio da organizacao ja saiu do estado puramente conceitual e ganhou:

* politica de sync;
* politica de seguranca;
* workflow de sync de metadados;
* docs de contribuicao e conduta;
* indexacao de documentacao.

O que ainda falta e transformar isso em regra operacional para forks, upstreams e releases curadas.

## 3. O que ainda precisa de ajuste

### 3.1 O catalogo nao deve viver "sozinho" no repo base

O rascunho antigo assumia um catalogo central como se tudo devesse ser mantido no mesmo lugar. Esse e o ponto que precisa de correcao conceitual.

O modelo mais util e este:

* o catalogo central descreve as integracoes;
* o catalogo aponta para a fonte real de cada integracao;
* a fonte real pode ser:
  * o upstream original;
  * um fork Orion;
  * uma release Orion curada;
* o repo base/original guarda a governance do sistema, nao toda a implementacao de cada plugin.

Isso evita que o Orion vire um deposito monolitico sem provenance clara.

### 3.2 Falta definir a estrategia por integracao

Cada integracao precisa de uma decisao explicita:

1. manter apenas como upstream original referenciado;
2. manter como fork Orion;
3. manter como fork Orion + release curada;
4. manter como upstream original, mas com metadados normalizados pelo Orion.

Hoje o codigo e a organizacao ja sugerem um modelo hibrido, mas a regra ainda nao esta formalizada em um documento unico.

### 3.3 O catalogo precisa guardar procedencia

O catalogo futuro nao deve guardar so nome e URL de download.

Ele precisa registrar algo como:

* id da integracao;
* nome de exibicao;
* repo upstream original;
* repo Orion, se existir;
* fonte de release;
* modo de instalacao;
* plataforma suportada;
* asset principal;
* checksum, quando houver;
* status de suporte;
* credito/autoria.

Sem isso, o Orion fica so como "lista de downloads" e nao como catalogo auditavel.

### 3.4 Os forks Orion precisam de governanca propria

Se uma integracao passar a viver como fork Orion, esse fork precisa carregar metadados de governanca.

Cada fork deveria explicar:

* qual e o upstream principal;
* qual e a finalidade do fork;
* o que o Orion altera;
* como contribuir de volta;
* qual e a fonte oficial de autoria;
* qual release e a confiavel para o usuario final.

### 3.5 Sync nao e release

O workflow de sync ja existe como ideia e como implementacao inicial, mas ele precisa continuar sendo tratado como sincronizacao de metadados e codigo, nao como publicacao automatica.

O fluxo ainda precisa deixar claro:

* sync pode atualizar dados;
* sync nao deve publicar release por padrao;
* release so ocorre depois de validacao;
* PR de sync deve ser revisado.

### 3.6 Ainda falta decidir a fonte de release por integracao

Para cada integracao, o Orion precisa responder:

* baixar do upstream original?
* baixar do fork Orion?
* baixar de uma release curada pelo Orion?
* permitir mais de uma opcao?

Esse ponto e crucial, porque ele define o que o catalogo significa na pratica.

## 4. Modelo recomendado

O modelo mais consistente continua sendo um **fork curado + upstream preservado**, mas com a ressalva de que nem toda integracao precisa começar como fork.

### Papel de cada camada

#### `.github`

Deve concentrar:

* policies;
* templates;
* workflow de sync;
* workflow de release;
* docs de governanca;
* regras de contribuicao;
* regras de seguranca.

#### `orion-interface`

Deve concentrar:

* UI;
* verificacao de releases;
* install flow;
* configuracao;
* validacao de integridade;
* acesso ao catalogo.

#### Repositorios de integracao

Devem concentrar:

* codigo do plugin;
* manifest;
* release artifacts;
* historico de upstream;
* credito aos autores.

### Estrutura de catalogo sugerida

```json
{
  "id": "steam",
  "name": "Steam",
  "platform": "steam",
  "upstreamRepo": "GOG-Nebula/galaxy-integration-steam",
  "orionRepo": "GOG-Orion/galaxy-integration-steam",
  "releaseSource": "orion-curated",
  "installMode": "gog-plugin-zip",
  "status": "supported",
  "assets": {
    "windows": "windows.zip"
  }
}
```

### Leituras possiveis de `releaseSource`

* `upstream-original`
* `orion-fork`
* `orion-curated`
* `mixed`

## 5. O que ainda merece verificacao

### 5.1 Repositorios declarados no frontend

O frontend ja aponta para Steam, Epic e Ubisoft, mas ainda precisa ficar claro quais desses repositorios existem de fato, quais sao forks Orion e quais ainda sao apenas placeholders de catalogo.

Se a UI listar uma integracao sem repo correspondente, a experiencia vira erro de runtime ou promessa falsa.

### 5.2 Regras de instalacao por plataforma

O fluxo atual ja bloqueia a instalacao com o cliente GOG aberto, o que faz sentido como regra de seguranca.

Ainda assim, vale manter em aberto:

* descoberta de caminho por plataforma;
* validacao de processo GOG por SO;
* comportamento quando o cliente nao e detectado com seguranca;
* mensagem de erro mais clara para o usuario.

### 5.3 Integridade de release

O checksum SHA-256 ja entrou no fluxo, mas o catalogo e o workflow ainda podem evoluir para:

* armazenar digest por asset;
* validar nomes de asset por plataforma;
* bloquear formatos nao esperados;
* registrar origem do asset em cada release.

### 5.4 Creditos, licencas e sustentacao

A centralizacao na organizacao Orion nao muda uma regra basica: o trabalho original dos autores externos continua sendo deles.

Isso precisa ficar claro em governanca, docs e conduta:

* dar credito explicito aos autores originais;
* respeitar a licenca de cada repositorio e de cada componente;
* manter PRs de melhoria abertos para os upstreams originais sempre que fizer sentido;
* nao apropriar autoria nem esconder a origem do trabalho;
* diferenciar curadoria de propriedade;
* tratar forks Orion como distribuicao e curadoria, nao como substituicao da autoria original.

O modelo de sustentacao tambem deve refletir isso:

* Orion nao deve ser tratado como software "pago para usar";
* o foco deve ser doacao e manutencao comunitaria;
* o acesso ao software e aos forks curados continua livre;
* doacoes servem para sustentar infraestrutura, manutencao e publicacao de releases.

Isso e importante para evitar ambiguidade entre "centralizar" e "tomar posse" do trabalho alheio.

## 6. Proximas etapas recomendadas

1. Formalizar o modelo de catalogo com `upstreamRepo`, `orionRepo`, `releaseSource` e `status`.
2. Definir, por integracao, se ela e upstream-only, fork Orion ou release curada.
3. Criar documentacao de governanca para cada fork Orion.
4. Garantir que essa governanca apareca tambem no Code of Conduct e nos docs de contribuicao.
5. Revisar o sync workflow para garantir que ele abra PR e nao publique release automaticamente.
6. Expandir a documentacao de procedencia, creditos e licencas.
7. Refinar a lista de integracoes reais, separando o que ja existe do que ainda e apenas plano.
8. Manter a regra de instalacao somente com o cliente GOG fechado.
9. Continuar melhorando a validacao de release, asset e checksum.

## 7. Conclusao

O Orion ja saiu da fase de corrigir bugs basicos do frontend e do instalador.

O proximo problema real e de arquitetura:

* como manter um catalogo unico;
* como preservar a origem dos repositorios;
* como decidir entre upstream original, fork Orion e release curada;
* como evitar que o repo base vire um monolito de integracoes sem provenance.

Essa revisao deve guiar os proximos passos sem perder a rastreabilidade dos autores originais nem a autonomia do Orion.
