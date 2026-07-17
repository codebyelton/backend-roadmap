# Funções em Rust 🛠️

Imagine que uma **função** é como um botão de um videogame. 🎮

Quando você aperta o botão, alguma coisa acontece.

Em Rust funciona igual: você cria uma função e pode usá-la sempre que precisar.

Isso evita escrever o mesmo código várias vezes.

---

# O que é uma função?

Uma função é um bloco de código que faz uma tarefa específica.

Toda função começa com a palavra `fn`.

Exemplo:

```rust
fn main() {
    println!("Olá, mundo!");
}
```

A função `main` é a mais importante do programa.

É nela que o Rust começa a executar o código.

---

# Criando uma função

Você pode criar suas próprias funções.

```rust
fn main() {
    saudacao();
}

fn saudacao() {
    println!("Olá!");
}
```

### Resultado

```
Olá!
```

Perceba que chamamos a função usando seu nome seguido de `()`.

```rust
saudacao();
```

---

# Onde posso criar uma função?

Em Rust, não importa se a função foi escrita antes ou depois da `main`.

Os dois exemplos abaixo funcionam.

```rust
fn main() {
    saudacao();
}

fn saudacao() {
    println!("Olá!");
}
```

ou

```rust
fn saudacao() {
    println!("Olá!");
}

fn main() {
    saudacao();
}
```

O importante é que ela exista em algum lugar do programa.

---

# Nomes das funções

Em Rust usamos o padrão **snake_case**.

Isso significa:

- Tudo em letra minúscula.
- Palavras separadas por `_`.

✅ Certo

```rust
mostrar_nome();
calcular_media();
somar_numeros();
```

❌ Errado

```rust
MostrarNome();
MostrarNomeUsuario();
```

---

# Funções com parâmetros 📦

Às vezes queremos enviar uma informação para a função.

Essa informação é chamada de **parâmetro**.

Exemplo:

```rust
fn main() {
    mostrar_numero(10);
}

fn mostrar_numero(numero: i32) {
    println!("{}", numero);
}
```

### Resultado

```
10
```

Perceba que em Rust sempre informamos o tipo do parâmetro.

```rust
numero: i32
```

---

# Vários parâmetros

Uma função pode receber várias informações.

```rust
fn main() {
    mostrar_pessoa("Maria", 20);
}

fn mostrar_pessoa(nome: &str, idade: i32) {
    println!("{} tem {} anos.", nome, idade);
}
```

### Resultado

```
Maria tem 20 anos.
```

Cada parâmetro possui seu próprio tipo.

---

# Retornando valores 🎁

Uma função também pode devolver um valor.

Para isso usamos `->`.

Exemplo:

```rust
fn cinco() -> i32 {
    5
}
```

Agora podemos usar esse valor.

```rust
fn main() {
    let numero = cinco();

    println!("{}", numero);
}
```

### Resultado

```
5
```

---

# O último valor é o retorno

Em Rust, normalmente **não usamos `return`**.

A última linha da função já é devolvida automaticamente.

Exemplo:

```rust
fn soma_um(numero: i32) -> i32 {
    numero + 1
}
```

### Resultado

Se chamarmos:

```rust
soma_um(5);
```

Recebemos:

```
6
```

---

# Cuidado com o ponto e vírgula ⚠️

Essa é uma das coisas mais importantes em Rust.

✅ Certo

```rust
fn soma() -> i32 {
    5
}
```

❌ Errado

```rust
fn soma() -> i32 {
    5;
}
```

O ponto e vírgula faz o Rust entender que aquilo é apenas uma instrução, e não um valor para retornar.

Se a função precisa devolver um valor, **a última expressão normalmente fica sem `;`**.

---

# O que são expressões?

Uma expressão é qualquer coisa que produz um valor.

Por exemplo:

```rust
5 + 3
```

O resultado dessa expressão é:

```
8
```

Outro exemplo:

```rust
let numero = 10;
```

O `10` é uma expressão.

As funções também podem usar expressões para retornar valores.

---

# Resumo Rápido 📝

- `fn` cria uma função.
- `main` é a primeira função executada.
- Você chama uma função usando `nome()`.
- Rust usa **snake_case** para nomear funções.
- Parâmetros recebem informações.
- Todo parâmetro precisa ter um tipo.
- `->` indica o tipo de retorno.
- A última expressão da função normalmente é o valor retornado.
- Se colocar `;` na última expressão, ela deixa de retornar um valor.

---

# Exercício 1 🧩

Crie uma função chamada:

```rust
boas_vindas()
```

Ela deve mostrar:

```
Bem-vindo ao Rust!
```

Depois chame essa função dentro da `main`.

---

# Exercício 2 🧩

Crie uma função chamada:

```rust
mostrar_idade(idade: i32)
```

Ela deve mostrar:

```
Você tem 18 anos.
```

Troque o valor passado para a função e veja o resultado mudar.

---

# Desafio Extra ⭐

Crie uma função chamada:

```rust
dobro(numero: i32) -> i32
```

Ela deve devolver o dobro do número recebido.

Exemplo:

```rust
fn main() {
    let resultado = dobro(8);

    println!("{}", resultado);
}
```

### Resultado esperado

```
16
```

💡 Esse desafio faz você praticar três coisas importantes ao mesmo tempo:

- Criar funções.
- Receber parâmetros.
- Retornar valores.