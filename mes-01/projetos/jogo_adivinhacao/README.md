# 🎯 Desafio: Jogo de Adivinhação em Rust

## Objetivo

O computador deve possuir um número secreto, e o jogador deverá tentar adivinhá-lo.

O jogo só termina quando o jogador acertar.

---

# 📋 Requisitos Obrigatórios

## 1. Gerar um número secreto

O programa deve possuir um número secreto entre **1 e 100**.

> **Dica:**
>
> Inicialmente, você pode definir o número manualmente.
>
> Exemplo:
>
> ```rust
> let numero_secreto = 37;
> ```
>
> Posteriormente, você poderá gerar esse número de forma aleatória.

---

## 2. Informar o objetivo do jogo

Ao iniciar o programa, exiba uma mensagem semelhante a:

```text
Bem-vindo ao jogo de adivinhação!

Tente descobrir o número entre 1 e 100.
```

---

## 3. Pedir um número ao jogador

Solicite que o usuário digite um número.

Exemplo:

```text
Digite um número:
```

---

## 4. Ler a entrada

Capture o valor digitado pelo usuário.

Você precisará converter o texto informado para um número inteiro.

---

## 5. Validar a entrada

Caso o usuário digite algo que não seja um número, o programa deve:

- Informar que a entrada é inválida.
- Solicitar um novo valor.
- Continuar executando normalmente, sem encerrar o programa.

Exemplo:

```text
Entrada inválida!

Digite novamente:
```

---

## 6. Criar um laço de repetição

Todo o jogo deverá acontecer dentro de um **loop**.

O programa deve continuar executando até que o jogador descubra o número secreto.

---

## 7. Comparar os números

Após cada tentativa, informe ao jogador:

### Se o número for menor que o número secreto

```text
Muito baixo!
```

### Se o número for maior que o número secreto

```text
Muito alto!
```

### Se acertar

```text
Parabéns!

Você acertou!
```

---

## 8. Encerrar o jogo

O laço de repetição deve terminar **somente** quando o jogador acertar o número.

---

# 🚀 Desafios Extras

## 🥇 Desafio 1 — Contador de Tentativas

Conte quantas tentativas o jogador realizou.

Ao acertar, exiba algo semelhante a:

```text
Parabéns!

Você acertou em 6 tentativas.
```

---

## 🥈 Desafio 2 — Avisar quando estiver perto

Caso a diferença entre o número digitado e o número secreto seja pequena, informe ao jogador.

Exemplo:

Número secreto:

```text
40
```

Jogador:

```text
38
```

Resposta:

```text
Está muito perto!
```

Defina você mesmo o que considera como "perto".

---

## 🥉 Desafio 3 — Validar números menores que 1

Não permita números menores que **1**.

Exemplo:

```text
-5
```

Resposta:

```text
Digite um número entre 1 e 100.
```

---

## 🏅 Desafio 4 — Validar números maiores que 100

Também não permita números maiores que **100**.

Exemplo:

```text
150
```

Resposta:

```text
Digite um número entre 1 e 100.
```

---

## 🎖️ Desafio 5 — Jogar novamente

Após o jogador vencer, pergunte se ele deseja iniciar uma nova partida.

Exemplo:

```text
Deseja jogar novamente?

1 - Sim
2 - Não
```

Se escolher **Sim**, o jogo reinicia.

Se escolher **Não**, o programa é encerrado.