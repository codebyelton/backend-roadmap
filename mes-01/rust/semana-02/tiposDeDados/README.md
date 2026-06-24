# Tipos de Dados

Existem dois grupos:

* Escalares
* Compostos

---

# Tipos Escalares

Guardam apenas um valor.

## Inteiros

São números sem vírgula.

```rust
let idade = 20;
```

Principais tipos:

| Tipo  | Tamanho               |
| ----- | --------------------- |
| i8    | 8 bits                |
| i16   | 16 bits               |
| i32   | 32 bits               |
| i64   | 64 bits               |
| i128  | 128 bits              |
| isize | Depende do computador |

Os tipos com `u` são sem sinal:

* `u8`
* `u16`
* `u32`
* `u64`
* `u128`
* `usize`

Exemplo:

```rust
let pontos: u8 = 100;
```

---

## Float

São números com vírgula.

```rust
let altura = 1.75;
```

Tipos:

* `f32`
* `f64` (mais usado)

Exemplo:

```rust
let peso: f64 = 75.5;
```

---

## Boolean

Só pode ter dois valores:

* `true`
* `false`

```rust
let vivo = true;
let morto = false;
```

---

## Char

Guarda apenas um caractere.

Usa aspas simples:

```rust
let letra = 'A';
let emoji = '😎';
```

---

# Tipos Compostos

Guardam vários valores.

## Tupla

Pode guardar tipos diferentes.

```rust
let pessoa = ("João", 20, true);
```

Pegando os valores:

```rust
println!("{}", pessoa.0);
println!("{}", pessoa.1);
println!("{}", pessoa.2);
```

---

## Array

Guarda vários valores do mesmo tipo.

```rust
let numeros = [10, 20, 30, 40];
```

Pegando um valor:

```rust
println!("{}", numeros[0]);
```

Saída:

```text
10
```

---

# Constantes

São valores que nunca mudam.

```rust
const PI: f64 = 3.14159;
```

Por convenção, constantes são escritas em MAIÚSCULO:

```rust
const VELOCIDADE_MAXIMA: u32 = 120;
```

---

# Resumão

### Inteiro

```rust
let pontos: i32 = 100;
```

### Float

```rust
let altura: f64 = 1.80;
```

### Boolean

```rust
let vivo = true;
```

### Char

```rust
let letra = 'A';
```

### Tupla

```rust
let pessoa = ("Maria", 25, true);
```

### Array

```rust
let numeros = [1, 2, 3];
```

# Exercício 1 - Tipos de Dados

Crie:

* Uma variável inteira chamada `vida`.
* Uma variável float chamada `altura`.
* Uma variável booleana chamada `jogando`.
* Uma variável char chamada `rank`.
* Uma tupla contendo nome, idade e nível.
* Um array com cinco números.

Depois, mostre todos os valores usando `println!`.
