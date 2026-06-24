# Variáveis e Tipos de Dados em Rust

## O que é uma variável?

Pensa em uma variável como uma **caixinha** que guarda informações.

Exemplo:

```rust
let nome = "João";
```

Aqui, a variável `nome` está guardando `"João"`.

---

# Criando variáveis

Usamos a palavra `let`:

```rust
let idade = 20;
```

---

# Variáveis são imutáveis por padrão

Depois de criar uma variável, você não pode mudar o valor dela:

```rust
let idade = 20;

idade = 21; // Erro!
```

Se quiser mudar o valor, use `mut`:

```rust
let mut idade = 20;

idade = 21;
```

Agora funciona!

---

# Dizendo o tipo da variável

Na maioria das vezes, Rust descobre sozinho:

```rust
let idade = 20;
```

Mas você também pode informar:

```rust
let idade: i32 = 20;
```

---

# Shadowing

Você pode criar outra variável com o mesmo nome:

```rust
let pontos = 10;

let pontos = pontos + 5;

println!("{}", pontos);
```

Saída:

```text
15
```
---

# Exercício 1 - Variáveis

Crie um programa que:

1. Crie uma variável chamada `nome`.
2. Crie uma variável mutável chamada `idade`.
3. Mude o valor da idade.
4. Mostre nome e idade usando `println!`.

Saída esperada:

```text
Nome: João
Idade: 21
```
