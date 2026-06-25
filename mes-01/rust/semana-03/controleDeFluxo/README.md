# Controle de Fluxo em Rust 🚦

Imagine que seu programa é como um jogo. Às vezes ele precisa tomar decisões:

- Se acontecer uma coisa, faça isso.
- Se não acontecer, faça outra coisa.

É aí que entra o **controle de fluxo**.

---

# if e else

O `if` serve para verificar uma condição.

👉 Se a condição for verdadeira (`true`), ele executa um bloco de código.

👉 Se for falsa (`false`), ele pode executar outro bloco usando `else`.

## Exemplo

```rust
fn main() {
    let idade = 12;

    if idade >= 10 {
        println!("Você já tem 10 anos ou mais!");
    } else {
        println!("Você tem menos de 10 anos!");
    }
}
```

### Resultado

```
Você já tem 10 anos ou mais!
```

---

# Importante ⚠️

Em Rust, a condição do `if` precisa ser um valor booleano (`true` ou `false`).

✅ Certo:

```rust
if numero > 0 {
    println!("Maior que zero");
}
```

❌ Errado:

```rust
if numero {
    println!("Maior que zero");
}
```

Rust não converte números automaticamente para `true` ou `false`.

---

# else if

Quando você precisa verificar várias condições, use `else if`.

## Exemplo

```rust
fn main() {
    let nota = 8;

    if nota == 10 {
        println!("Perfeito!");
    } else if nota >= 7 {
        println!("Aprovado!");
    } else {
        println!("Reprovado!");
    }
}
```

### Resultado

```
Aprovado!
```

---

# if dentro de uma variável

Em Rust, o `if` também pode devolver um valor.

## Exemplo

```rust
fn main() {
    let dia_ensolarado = true;

    let passeio = if dia_ensolarado {
        "Parque"
    } else {
        "Casa"
    };

    println!("Vamos para {}", passeio);
}
```

### Resultado

```
Vamos para Parque
```

⚠️ Os dois lados (`if` e `else`) precisam devolver o mesmo tipo de dado.

✅ Certo:

```rust
let numero = if true { 5 } else { 10 };
```

❌ Errado:

```rust
let numero = if true { 5 } else { "dez" };
```

Um é número e o outro é texto.

---

# Match

O `match` serve para comparar valores e escolher uma ação.

Ele é parecido com vários `if else`, mas geralmente fica mais organizado.

## Exemplo

```rust
fn main() {
    let ataque = "magia";

    match ataque {
        "espada" => println!("Dano: 10"),
        "arco" => println!("Dano: 7"),
        "magia" => println!("Dano: 15"),
        _ => println!("Ataque inválido"),
    }
}
```

### Resultado

```
Dano: 15
```

O símbolo `_` significa:

> "Se não for nenhum dos casos acima, faça isso."

---

# Resumo Rápido 📝

- `if` → verifica uma condição.
- `else` → executa quando a condição é falsa.
- `else if` → verifica outras condições.
- O `if` precisa receber `true` ou `false`.
- O `if` pode devolver valores.
- `match` é ótimo para comparar vários valores diferentes.

---

# Exercício 1 - If e Else

Crie um programa que tenha a variável:

```rust
let idade = 15;
```

Faça o programa mostrar:

- `"Pode entrar"` se a idade for 18 ou mais.
- `"Não pode entrar"` caso contrário.

### Resultado esperado

```
Não pode entrar
```

---

# Exercício 2 - Match

Crie uma variável:

```rust
let animal = "gato";
```

Use `match` para mostrar:

- `"Miau"` para gato.
- `"Au Au"` para cachorro.
- `"Piu Piu"` para passarinho.
- `"Animal desconhecido"` para qualquer outro valor.

### Resultado esperado

```
Miau
```

---

# Desafio Extra ⭐

Tente mudar os valores das variáveis e veja como a saída muda.

É uma das melhores formas de aprender Rust!