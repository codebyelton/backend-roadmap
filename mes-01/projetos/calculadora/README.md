# Projeto: Calculadora em Rust 🧮

Neste projeto, você vai criar uma calculadora simples em Rust.

A calculadora deverá realizar:

- ➕ Soma
- ➖ Subtração
- ✖️ Multiplicação
- ➗ Divisão

Você vai construir tudo aos poucos. Não tente fazer tudo de uma vez. 🚀

---

# 🎯 Objetivo do Projeto

Ao final, sua calculadora deverá:

- Mostrar um menu.
- Permitir que o usuário escolha uma operação.
- Receber dois números.
- Realizar a operação escolhida.
- Mostrar o resultado.
- Voltar ao menu depois da operação.
- Permitir que o usuário encerre o programa.

---

# 🧩 Etapa 1: Criar o projeto

Crie um novo projeto Rust chamado:

```text
calculadora
```

Depois execute o projeto para garantir que ele está funcionando.

### Seu objetivo 🎯

Faça o programa mostrar uma mensagem de boas-vindas.

Exemplo:

```text
Bem-vindo à calculadora!
```

---

# 🧩 Etapa 2: Criar o menu

Crie um menu mostrando as opções disponíveis:

```text
=== CALCULADORA ===

1 - Somar
2 - Subtrair
3 - Multiplicar
4 - Dividir
5 - Sair
```

### Seu objetivo 🎯

Faça o menu aparecer corretamente no terminal.

💡 Ainda não precisa fazer as opções funcionarem.

---

# 🧩 Etapa 3: Receber a escolha do usuário

Agora faça o programa esperar o usuário digitar uma opção.

O programa deve:

1. Criar uma variável para guardar a escolha.
2. Ler o que o usuário digitou.
3. Mostrar a escolha na tela.

### Exemplo

```text
Escolha uma opção:
1

Você escolheu: 1
```

💡 Lembre-se: o que o usuário digita chega como texto.

---

# 🧩 Etapa 4: Usar match

Agora use `match` para verificar a escolha do usuário.

Crie uma ação para cada opção:

- `1` → Mostrar que a pessoa escolheu soma.
- `2` → Mostrar que escolheu subtração.
- `3` → Mostrar que escolheu multiplicação.
- `4` → Mostrar que escolheu divisão.
- `5` → Mostrar que deseja sair.
- Qualquer outra opção → Mostrar que é inválida.

### Seu objetivo 🎯

Faça cada opção mostrar uma mensagem diferente.

---

# 🧩 Etapa 5: Criar a função de soma

Crie uma função que:

- Receba dois números.
- Some os dois.
- Retorne o resultado.

### Exemplo de funcionamento

```text
Número 1: 10
Número 2: 5

Resultado: 15
```

💡 Pense:

- Quais parâmetros a função precisa?
- Qual tipo de dado os números terão?
- Qual tipo de dado a função deve retornar?

---

# 🧩 Etapa 6: Criar as outras operações

Agora crie uma função para cada operação:

### Subtração

A função deve:

- Receber dois números.
- Subtrair o segundo do primeiro.
- Retornar o resultado.

---

### Multiplicação

A função deve:

- Receber dois números.
- Multiplicar os valores.
- Retornar o resultado.

---

### Divisão

A função deve:

- Receber dois números.
- Dividir o primeiro pelo segundo.
- Retornar o resultado.

---

# 🧩 Etapa 7: Criar uma função para ler números

Agora crie uma função responsável por ler um número digitado pelo usuário.

Essa função deverá:

1. Criar uma variável para guardar o texto.
2. Ler a entrada do usuário.
3. Remover espaços e quebras de linha.
4. Converter o texto para um número.
5. Retornar esse número.

💡 Você precisará pesquisar e usar:

- `stdin`
- `read_line`
- `trim`
- `parse`

---

# 🧩 Etapa 8: Pedir os dois números

Quando o usuário escolher uma operação, o programa deverá:

1. Pedir o primeiro número.
2. Guardar o valor.
3. Pedir o segundo número.
4. Guardar o valor.

### Exemplo

```text
Digite o primeiro número:
10

Digite o segundo número:
5
```

---

# 🧩 Etapa 9: Conectar as funções

Agora conecte tudo.

Quando o usuário escolher:

```text
1
```

O programa deve:

1. Pedir dois números.
2. Chamar a função de soma.
3. Receber o resultado.
4. Mostrar o resultado.

Faça o mesmo para:

- Subtração.
- Multiplicação.
- Divisão.

💡 Aqui você vai praticar bastante a comunicação entre funções.

---

# 🧩 Etapa 10: Tratar divisão por zero ⚠️

Antes de realizar uma divisão, verifique o segundo número.

Se ele for zero:

```text
Não é possível dividir por zero!
```

Caso contrário:

```text
Resultado: ...
```

💡 Use `if` e `else` para tomar essa decisão.

---

# 🧩 Etapa 11: Criar um loop

Agora faça a calculadora continuar funcionando depois de uma operação.

O fluxo deve ser:

```text
Mostrar menu
↓
Escolher operação
↓
Digitar números
↓
Calcular
↓
Mostrar resultado
↓
Voltar ao menu
```

💡 Use um `loop`.

---

# 🧩 Etapa 12: Criar a opção Sair

Quando o usuário escolher:

```text
5
```

O programa deverá:

1. Mostrar uma mensagem de despedida.
2. Encerrar o loop.

💡 Use `break`.

---

# 🏆 Desafio Final

Agora tente terminar a calculadora sem olhar exemplos prontos.

Ela deve:

- [ ] Mostrar um menu.
- [ ] Receber a escolha do usuário.
- [ ] Usar `match`.
- [ ] Receber dois números.
- [ ] Ter uma função de soma.
- [ ] Ter uma função de subtração.
- [ ] Ter uma função de multiplicação.
- [ ] Ter uma função de divisão.
- [ ] Impedir divisão por zero.
- [ ] Usar um `loop`.
- [ ] Permitir sair do programa.
- [ ] Mostrar o resultado da operação.

---

# 🚀 O que você praticou?

Durante o projeto, você vai praticar:

- Variáveis.
- Funções.
- Parâmetros.
- Valores de retorno.
- `String`.
- `f64`.
- Entrada de dados.
- `match`.
- `if` e `else`.
- `loop`.
- `break`.
- Conversão de texto para número.

💡 Não tenha pressa. Se travar em uma etapa, tente resolver apenas aquele pequeno problema antes de continuar.

O objetivo não é copiar uma calculadora pronta.

O objetivo é construir a sua própria. 🧮🚀