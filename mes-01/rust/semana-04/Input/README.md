# Input em Rust

## O que é input?

Input é quando o programa faz uma pergunta e espera você digitar a
resposta.

Fluxo:

``` text
Teclado
  ↓
stdin()
  ↓
read_line()
  ↓
Variável
```

------------------------------------------------------------------------

## O que é `mut`?

Pense que uma variável é uma caixa.

Sem `mut`:

``` rust
let nome = String::new();
```

A caixa está fechada. Ninguém pode colocar nada nela.

Com `mut`:

``` rust
let mut nome = String::new();
```

Agora a caixa está aberta e pode receber um valor.

**Resumo:** `mut` = "essa variável pode mudar".

------------------------------------------------------------------------

## O que é `read_line()`?

Ele faz exatamente isso:

> "Leia o que a pessoa digitou e coloque dentro da variável."

``` rust
let mut nome = String::new();

std::io::stdin().read_line(&mut nome);
```

Se você digitar:

``` text
Elton
```

A variável fica assim:

``` text
"Elton\n"
```

Esse `\n` é o Enter.

------------------------------------------------------------------------

## O que é `&mut`?

Significa:

> "Pode modificar essa variável."

Sem isso, o Rust não deixa o `read_line()` colocar nada na variável.

------------------------------------------------------------------------

## O que faz `trim()`?

Remove o Enter (`\n`) e espaços extras.

``` rust
nome.trim()
```

Antes:

``` text
"Elton\n"
```

Depois:

``` text
"Elton"
```

------------------------------------------------------------------------

## O que faz `parse()`?

Tudo que vem do teclado é texto.

Se quiser um número, precisa converter.

``` rust
let idade: u32 = idade.trim().parse().unwrap();
```

------------------------------------------------------------------------

# Exemplo 1 - Recebendo um nome

``` rust
use std::io;

fn main() {
    let mut nome = String::new();

    println!("Digite seu nome:");

    io::stdin().read_line(&mut nome);

    println!("Olá, {}", nome.trim());
}
```

------------------------------------------------------------------------

# Exemplo 2 - Recebendo uma idade

``` rust
use std::io;

fn main() {
    let mut idade = String::new();

    println!("Digite sua idade:");

    io::stdin().read_line(&mut idade);

    let idade: u32 = idade.trim().parse().unwrap();

    println!("Você tem {} anos.", idade);
}
```

------------------------------------------------------------------------

# Resumo para decorar

-   `stdin()` → recebe o que você digitou.
-   `read_line()` → coloca o texto dentro da variável.
-   `mut` → deixa a variável mudar.
-   `&mut` → dá permissão para modificar a variável.
-   `trim()` → tira o Enter.
-   `parse()` → transforma texto em número.

------------------------------------------------------------------------

# Exercícios

## 1. Nome

Faça um programa que:

-   Pergunte o nome do usuário.
-   Guarde o nome.
-   Mostre:

``` text
Olá, João!
```

------------------------------------------------------------------------

## 2. Idade

Faça um programa que:

-   Pergunte a idade.
-   Converta para número.
-   Mostre:

``` text
Você tem 18 anos.
```

------------------------------------------------------------------------

## 3. Nome + Idade

Faça um programa que pergunte:

-   Nome
-   Idade

Depois mostre:

``` text
Olá, João!
Você tem 18 anos.
```

**Desafio:** use `trim()` nos dois inputs e `parse()` apenas na idade.
