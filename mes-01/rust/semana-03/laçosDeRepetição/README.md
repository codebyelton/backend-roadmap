# Laços de Repetição em Rust 🔄

Às vezes queremos executar o mesmo código várias vezes.

Imagine que você quer:

- Contar de 1 até 10.
- Mostrar uma mensagem várias vezes.
- Percorrer uma lista de itens.

Para isso usamos os **laços de repetição (loops)**.

Rust possui 3 tipos principais:

- `loop`
- `while`
- `for`

---

# loop

O `loop` é o laço mais simples.

Ele repete para sempre até você mandar parar.

## Exemplo

```rust
fn main() {
    loop {
        println!("Olá!");
    }
}
```

### Resultado

```
Olá!
Olá!
Olá!
Olá!
...
```

⚠️ Esse programa nunca para sozinho.

Para sair de um `loop`, usamos `break`.

## Exemplo com break

```rust
fn main() {
    let mut contador = 0;

    loop {
        println!("{}", contador);

        contador += 1;

        if contador == 5 {
            break;
        }
    }
}
```

### Resultado

```
0
1
2
3
4
```

Quando o contador chega em 5, o `break` encerra o laço.

---

# while

O `while` significa:

> "Enquanto essa condição for verdadeira, continue repetindo."

## Exemplo

```rust
fn main() {
    let mut numero = 3;

    while numero > 0 {
        println!("{}", numero);

        numero -= 1;
    }

    println!("Fim!");
}
```

### Resultado

```
3
2
1
Fim!
```

O laço continua enquanto `numero > 0`.

Quando a condição fica falsa, ele para.

---

# for

O `for` é o laço mais usado em Rust.

Ele serve para percorrer coleções como arrays e listas.

## Exemplo

```rust
fn main() {
    let numeros = [10, 20, 30];

    for numero in numeros {
        println!("{}", numero);
    }
}
```

### Resultado

```
10
20
30
```

O `for` pega cada item da coleção automaticamente.

Você não precisa controlar índices nem contadores.

Por isso ele é mais seguro e mais fácil de ler.

---

# For com Intervalos

Também podemos usar o `for` para repetir uma quantidade de vezes.

## Exemplo

```rust
fn main() {
    for numero in 1..6 {
        println!("{}", numero);
    }
}
```

### Resultado

```
1
2
3
4
5
```

⚠️ O último número não é incluído.

O intervalo `1..6` significa:

```
1 até antes do 6
```

---

# Contagem Regressiva

Podemos inverter um intervalo usando `.rev()`.

## Exemplo

```rust
fn main() {
    for numero in (1..4).rev() {
        println!("{}", numero);
    }

    println!("LIFTOFF!!!");
}
```

### Resultado

```
3
2
1
LIFTOFF!!!
```

O método `.rev()` simplesmente inverte a ordem.

---

# Qual usar?

| Laço | Quando usar |
|--------|--------|
| `loop` | Quando você quer repetir indefinidamente até usar `break`. |
| `while` | Quando existe uma condição para continuar repetindo. |
| `for` | Quando você quer percorrer listas, arrays ou repetir um número de vezes. |

👉 Na maioria dos casos, o `for` é a melhor escolha.

---

# Resumo Rápido 📝

- `loop` repete para sempre.
- `break` encerra um laço.
- `while` executa enquanto uma condição for verdadeira.
- `for` percorre coleções e intervalos.
- `for` é o laço mais usado em Rust.
- `.rev()` inverte a ordem de um intervalo.

---

# Exercício 1 - Loop

Crie um programa que:

- Comece com a variável:

```rust
let mut contador = 1;
```

- Use um `loop`.
- Mostre os números de 1 até 5.
- Use `break` para parar o laço.

### Resultado esperado

```
1
2
3
4
5
```

---

# Exercício 2 - While

Crie um programa que:

- Comece com:

```rust
let mut energia = 5;
```

- Enquanto a energia for maior que 0:
  - Mostre a energia.
  - Diminua 1 ponto.

- Quando acabar, mostre:

```text
Sem energia!
```

### Resultado esperado

```
5
4
3
2
1
Sem energia!
```

---

# Exercício 3 - For

Crie um array:

```rust
let frutas = ["Maçã", "Banana", "Uva"];
```

Use um `for` para mostrar cada fruta.

### Resultado esperado

```
Maçã
Banana
Uva
```

---

# Desafio Extra ⭐

Faça uma contagem regressiva de 10 até 1 usando:

```rust
for numero in (1..11).rev()
```

No final mostre:

```text
Foguete lançado! 🚀
```