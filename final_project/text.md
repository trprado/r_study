# Preços de computadores pessoais
## Análise univariada
### Qual é a estrutura dos dados?
Os dados analisados são provenientes do pacote Ecdat do software R (R Core Team, 2019). Eles correspondem às informações de vendas de computadores pessoais que ocorreram entre os períodos de Janeiro de 1993 até Novembro de 1995.

As variáveis disponíveis para as análises são:
price: (int) preço em dólares americanos de computadores 486.
speed: (int) velocidade do clock em MHz do processador
hd: (int) tamanho do Hard Disk (HD) em MB.
ram: (int) tamanho da memória de acesso randômico (RAM) em MB
screen: (int) tamanho dos monitores de tubo em polegadas.
cd: (Factor) presença ou ausência de leitora de CDROM.
multi: (Factor) presença ou ausência de kit multimídia (auto falantes, placa de som).
premium: (Factor) informações do fabricante dos computadores categorizadas por marca conhecida (yes), como IBM ou COMPAQ, ou não (no).
trend: (int) tempo em meses de Janeiro de 1993 até Novembro de 1995.
date: (Date) data das vendas dos computadores por anos e meses.

### Quais são as características mais importantes do dataset?
As características que representam a maior importância são *price* e *ram*. Gostaria de encontrar características que possam ser utilizadas para determinar o preço de um computador. Também suspeito que a *ram* e outras combinações de variáveis possam ser utilizadas para criar um modelo preditivo que ajude a determinar os preço de um computador entre 1993 até 1995. Outros anos não irão ser considerar, pois é preciso explorar nesse momento outras variáveis que o dataset não possui.

### Outras características que penso que podem ajudar na investigação?
As variáveis *ram*, *hd*, *speed* e as categóricas podem contribuir para determinar o preço de um computador dentro da época em que os dados foram capturados. Penso que *ram* com *speed* possam ser de maior importância na contribuição por serem o que torna um computador em si mais rápidos, porém na época HDs também contribuíam para um alto preço, visto que o armazenamento era escasso.

### Foi criado novas variáveis com base no dataset?
Foi criado *date* que contém a data catalogada do preço de cada computador, ela foi utilizada pois *trend* que mostra apenas a contagem de meses a partir de janeiro de 1993 não é muito explicativa a humanos, assim ao utilizar datas temos uma visualização informativa e legível.

### Das características investigadas, existe alguma distribuição não usual?
Não foi incluso na análise a variável *ads* (número de vezes que o valor do produto foi listado por cada mês) pois foi considerada redundante para as análises. O ano de 1995 possui apenas dados até novembro, o que acaba não informando as vendas durante a época de festas de dezembro que poderia conter um maior número de vendas.

Após criada a variável auxiliar *date* os dados possuíam dimensão de 6529 observações e 10 variáveis.

## Análise Univariada
No ano de 1993 o preço médio U\$ 2340 sendo que 90,96% foram vendidos por empresas consideradas premium, sendo que a a maioria dos computadores vendidos desse ano tem as seguintes características:

- Processador de 33MHz;
- 340MB de espaço em HD;
- 4MB de memória RAM;
- Telas de tubo de 14";
- Poucos computadores possuíam drive de CDROM;
- A grande maioria não possuía especificações multimídia, provavelmente devido ao seu alto custo.

Para o ano de 1994 o preço médio dos computadores era de U\$ 2196, com uma diminuição no preço comparado ao ano anterior. Nesse ano houve uma queda de aproximadamente 3.38% das vendas de computadores por empresas *premium*. Os computadores desse ano possuíam as seguintes características:

- Continuam a ser a grande maioria os processadores com 33 MHz. Esse pode ser um dos motivos do menor custo dos computadores no ano de 94;
- 340MB de espaço em HD, sugerindo outro motivo para a diminuição do valor médio dos computadores;
- Houve o aumento em 4MB de memória RAM no ano de 1994, totalizando 8MB de RAM;
- As telas continuam a ser de 14";
- Houve um aumento no interesse de computadores com drive de CDROM;
- Para esse ano, houve um aumento no interesse geral de suporte multimídia, porém ainda não esta presente na maioria dos computadores.

Para o ano de 1995, o último ano de estudo e avaliação do perfil de venda dos computadores nos EUA, tem preço médio de U\$ 2015. Nesse ano as vendas de computadores por empresas *premium* aumentou para 96.02%, mostrando um domínio quase completo das vendas. As suas configurações são:

- Aumento no processador para 66Mhz;
- 1000MB de espaço em disco, o que representa um grande salto de armazenamento físico;
- Se mantêm os 8MB de RAM;
- As telas passam a ser de 15" na maioria dos computadores vendidos;
- A grande maioria dos computadores vendidos possuía drive de CDROM;
- O ano de 1995 representa um aumento geral de itens multimídia presentes, fazendo parte da maioria dos computadores.

O ano de 1995 representa uma queda no valor médio dos computadores com aumentos em processamento, espaço de armazenamento e tamanho da tela de tubo. Também, todos os computadores que apresentaram possuir CDROM possuíam também o kit multimídia.

Monitores não têm informação se são coloridos ou de fósforo verde.

## Análise Bivariada
A primeira figura (Figura 1) é um *Heatmap* com a correlação entre as variáveis onde pudemos observar que, as variáveis *hd* e *ram* possuem uma forte correlação positiva, o que pode ser observado na Figura 13, também temos uma boa correlação entre *ram* e *price*.

Os mesmos pontos podem ser vistos na Figura 2, onde temos um gráfico da matriz de correlação com dispersão e na diagonal principal o histograma da distribuição de cada variável.

## Análise Multivariada
Na Figura 5 o gráfico de dispersão nos ajuda a observar a tendencia monetaria dos custos de computadores por ano com relação a velocidade de processamento da CPU. Em 1993 os computadores possuiam até 66MHz de processamento enquanto a partir de 1994 foram lançados processador com velocidades de até 100MHz e com um custo menor ou equivalente a processadores do ano anterior e com menor frequência. Em 1995, computadores com processadores de até 100MHz custava um pouco mais que U\$ 2000.

Já o gráfico da Figura 6 representa a dispersão entre *log10(price)* e *hd*. No ano de 1993 podemos ver que as maiorias dos computadores possuíam HDs entre 80MB até 580MB, sendo que a maioria como foi validado anteriormente possuíam 340MB. Seus preços estavam entre valor de U\$ 2000 até U\$ 5000 dependendo de suas características. No ano de 1994 essa característica se mantém, com valor entre 80MB até 580MB de espaço de armazenamento, sendo que seus valores se mantiverem entre os U\$ 2000 a U\$ 5000. Em 1995 os HDs passam a ter um maior tamanho, sendo que os dados não informam vendas de computadores com menos de 180MB e os preços dos computadores despencam, com um custo mediano a baixo de U\$ 2000.

A figura 7 apresenta a dispersão de *log10(price)* pela *RAM*. A memória RAM normalmente é vinculada a forma $2^{i}$ com $i = 1, 2, 3, ..., n$. Nesses dados têm valores entre 2MB até 32MB de memória *RAM*, sendo que em 1993 os valores estavam entre 4MB a 8MB em computadores com valores entre U\$ 2000 a U\$ 4000, alguns computadores até U\$ 5000 podiam vir com até 16MB, com raros casos de maior quantidade. Já o ano de 1994 não houve grandes alterações, com alguns computadores podendo conter memórias até 24MB e valores inferiores a U\$ 4000. Em 1995 os computadores com quantidades de até 32MB de memória custavam pouco mais de U\$ 3000.

Podemos ver a tendência dos valores medianos entre os anos de 1993 até 1995 na Figura 8, onde temos que 90% dos computadores custam até pouco mais de U\$ 3000 dependendo da época do ano, enquanto em média seu valor fica entre U\$ 2000 e U\$ 2500. Podemos ver um aumento súbito do valor monetário de computadores no ano de 1995, entre os meses de setembro até outubro, permanecendo constante até novembro, sendo que esta é uma eṕoca nos EUA que se tem uma baixa nos preços devido ao *Black Friday* ao contrário do ano anterior que houve uma queda dos preços referente ao mesmo período. Podemos pensar que uma possível razão para isso, é que o aumento da procura por computadores devido ao fato de sua popularização e as festividades de final de ano possam ter gerado um aumento na oferta e demanda.

A Figura 9 nos mostra uma tendência onde computador vendidos por marcas *premium* tem um menor custo comparados a computadores sem marca ou marcas não consideradas *premium*. Essa tendência e confirmada entre março de 1993 até setembro de 1995, onde vemos computadores com marca sendo comercializados com valores muito acima daqueles sem marca. Empresas *premium* costumam ter uma produção maior de computadores do que aquelas que não são *premium*, fazendo computadores terem um custo menor por unidade vendida, por isso se modifica no final de 1995 o que pode significar que o custo de marca esta sendo incluindo no valor desses computadores vendidos.

Na Figura 10, a partir de Março de 1993, podemos observar o aumento da procura do consumidor por computadores que possuíam drive de CD, sendo que esses possuíam valores maiores do que aqueles sem o periférico. Observa-se uma queda nos preços de computadores que não possuem o drive durante a passagem dos anos. Já os computadores que possuíam o drive tiveram uma queda de preço no início de 1995 porém ao final do mesmo ano os valores aumentaram expressivamente.

No primeiro semestre de 1993, os computadores eram comercializados sem *kit* multimídia, como pode ser observado na Figura 11. A partir de Julho do mesmo ano, começou a venda de computares com *kit* multimídia. Entretanto, o valor das vendas, embora seja um pouco maior para os computadores que possuíam o *kit*, não diferenciou muito dos computadores que não possuíam o *kit*. No final de 1995, nos meses de Agosto até Novembro, observa-se que houve um aumento no preço dos eletrônicos que tanto possuíam ou não o *kit*, porém os que possuíam o recurso de multimídia apresentavam um custo maior do que os que careciam desse recurso.

Já na figura 12 temos um arranjo de *boxplots* com as variáveis pelo *log10(price)*. Com relação a velocidade da *CPU*, podemos ver que a mediana dos preços se encontra com valores a baixo de U\$ 3000, independente da velocidade. Observa-se que as únicas diferenças que aparentam ser significativas esta entre a velocidade de 100MHz e a de 25MHz em relação ao preço. O mesmo pode ser observado para a memória *RAM*, a partir do momento que temos um aumento na quantidade de memória, temos um aumento no preço do computador.

As demais variáveis não parece ter uma influência significativa no preço final do produto.

A Figura 13 é outro gráfico de *boxplot* com relação entre o *log10(price)* e *hd*. Nele podemos ver pouca diferença do preço de computadores com armazenamento interno entre 85MB a 320MB, sendo que computadores com 80MB de armazenamento tem um custo bem próximo U\$ 1000. Nota-se que para alguns computadores computadores com *HDs*  525MB a 1370MB apresentaram os maiores valores monetários em 1993. Já em 1994 poucos computadores tiveram preços acima de U\$ 4000 com exceção de alguns computadores que possuíam *HD* de 728MB e 1000MB.

Em 1995 computadores que possuíam *HDs* com tamanhos de 1600MB e 2100MB são os que apresentaram o maior valor monetário.

Pela Figura 14 podemos observar que a medida que o tamanho do *HD* aumenta a quantidade de memória *RAM* também tende a aumentar. Com exceção do *HD* de 2100MB que possuía 16MB de *RAM*.

Pela Figura 15 observamos que as marcas *premium* vendiam computadores com *kit* multimídia e drive de *CD* com maior frequência e preços menores que aqueles sem marca *premium*. Observa-se também que computadores que não possuíam drives de *CD* não eram vendidos com *kit* multimídia.

## Modelo de regressão linear

## Gráficos finais e sumário
### Primeiro gráfico escolhido: Distribuição de densidade do preço de computadores 486, correspondente aos anos de 1993 até 1995
Apesar de ter transformado a variável resposta em logaritmo, ainda assim ela não apresenta uma distribuição normal (p-valor < 0,05) optou-se por fazer uma análise de modelo linear simples pois graficamente a distribuição do logaritmo do preço adquire aproximadamente uma forma simétrica de sino.

### Segundo gráfico escolhido: Boxplot de ausência ou não de CD por preço, agrupado por kit multimídia
A figura acima foi documentada na análise multivariada demonstrando a não significância estatística das variáveis *cd* e *multi* como explicativas do preço dos computadores 486, como foi comprovado na análise de modelos.

### Terceiro gráfico escolhido: Equação do modelo proposto
Apesar do efeito da multicolinearidade e da falta de distribuição de normalidade da variável resposta, a figura acima demonstra que o modelo proposto se ajusta satisfatoriamente aos dados.

## Reflexões
O dataset de computadores contém 6259 observações de 10 variáveis, dessas não existem valores nulos e os dados representam computadores entre os anos de 1993 até 1995. Inicialmente, foram feitas análises tabulares, de sumário e gráficas para analisar o perfil do consumidor nos anos correspondentes, assim como avaliar o comportamento das variáveis.

Na análise bidimensional, verificou-se uma alta correlação entre as variáveis *hd* e *ram* de 80%, e correlações moderadas para as variáveis *price* e *ram* e *hd* com *trend*, ambas com 60%. Na análise multivariada podemos ver as tendências de comportamento entre as variáveis com a variável dependente preço, tanto *speed*, *hd*, *ram* e *screen* mostraram tendências positivas no aumento dos preços dos computadores, já as variáveis *multi* e *cd* não apresentaram tendência nenhuma. O preço dos computadores apresenta um decrescimento com o passar dos anos, com exceção do final de 1995 onde o valor do produto tem um aumento súbito, razão não encontrada nos dados. Após transformar a variável *price* em logaritmo e torná-la o mais o mais próximo de uma distribuição norma, obteve-se 54.4% da variância do modelo, motivo para tal, é que talvez haja variáveis explicativas não observadas e que influenciem na variável resposta.

Alguma das limitações encontradas nos dados, é que em primeiro ponto, estamos trabalhando com dados de quase 30 anos atrás. Esses dados não contam com ajustes de inflação, juros e outros ajustes monetários. Existe também um salto de desempenho e configurações entre esse tempo não catalogado. Também temos de considerar que para os dias atuais, alguns dos *hardwares* começaram a se tornar obsoletos, onde podemos observar que no tempo atual *HDs* estão sendo alterados por *SSDs* e a troca de telas de tubo por monitor LCD e outras tecnologias, demonstrando que o perfil do consumidor se alterou com o tempo, sendo que entre 1993 a 1995 computadores ainda eram considerados artigo de luxo, refletindo que o presente estudo é transversal. Embora atualmente seja acessível ter um computador, o *hardware* evolui de tal maneira que apenas os itens citados no banco de dados não são suficientes para estimar nos dias atuais o preço final de um computador.
