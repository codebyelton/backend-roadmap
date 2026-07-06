# Variáveis em C# 📦

Imagine que uma **variável** é como uma **caixa**.

Você coloca alguma coisa dentro dela (um número, um texto, uma letra...) e depois pode usar essa informação sempre que precisar.

Exemplo:

📦 Caixa chamada `nome`

Dentro dela:

```
Elton
```

Sempre que você usar `nome`, o programa vai lembrar que ali está `"Elton"`.

---

# Criando uma variável

Em C#, para criar uma variável, precisamos dizer duas coisas:

- O tipo da informação.
- O nome da variável.

Exemplo:

```csharp
string nome = "Elton";
```

Aqui temos:

- `string` → Tipo (texto)
- `nome` → Nome da variável
- `"Elton"` → Valor armazenado

---

# Os principais tipos de variáveis

## string 📝

Guarda textos.

```csharp
string nome = "Maria";
```

Também pode guardar frases.

```csharp
string mensagem = "Olá, mundo!";
```

---

## int 🔢

Guarda números inteiros.

```csharp
int idade = 20;
```

Pode ser:

```csharp
int pontos = 150;
```

Mas não pode ter vírgula.

❌ Errado

```csharp
int preco = 10.5;
```

---

## double 📏

Guarda números com casas decimais.

```csharp
double altura = 1.75;
```

Outro exemplo:

```csharp
double peso = 82.4;
```

---

## char 🔠

Guarda apenas **um único caractere**.

Sempre usa aspas simples.

```csharp
char letra = 'A';
```

Pode ser um número também.

```csharp
char numero = '7';
```

⚠️ Só pode guardar UM caractere.

---

## bool ✅❌

Guarda apenas dois valores:

- `true`
- `false`

Exemplo:

```csharp
bool ligado = true;
```

Ou

```csharp
bool terminou = false;
```

Muito usado em condições (`if`).

---

# Exibindo uma variável

Usamos o `Console.WriteLine()`.

```csharp
string nome = "Elton";

Console.WriteLine(nome);
```

Resultado:

```
Elton
```

Também podemos juntar texto com variáveis.

```csharp
string nome = "Elton";

Console.WriteLine("Olá " + nome);
```

Resultado:

```
Olá Elton
```

---

# Alterando uma variável

O valor pode mudar.

```csharp
int vidas = 3;

vidas = 2;

Console.WriteLine(vidas);
```

Resultado:

```
2
```

A variável continua sendo a mesma.

Só o valor mudou.

---

# var 🤖

Às vezes o C# consegue descobrir sozinho o tipo da variável.

Nesse caso usamos `var`.

```csharp
var nome = "Elton";
```

O C# entende que isso é uma `string`.

Outro exemplo:

```csharp
var idade = 20;
```

Ele entende que é um `int`.

💡 No começo dos estudos, é uma boa ideia escrever o tipo (`string`, `int`, `double`...) para ficar mais fácil de aprender.

---

# Regras para criar variáveis 📌

✅ Pode:

```csharp
string nome;
int idade;
double salario;
```

❌ Não pode começar com número.

```csharp
int 1idade;
```

❌ Não pode ter espaço.

```csharp
string meu nome;
```

✅ O mais comum é usar o padrão **camelCase**.

```csharp
string nomeCompleto;
int quantidadeDeVidas;
double notaFinal;
```

---

# Resumo Rápido 📝

| Tipo | Guarda |
|------|---------|
| `string` | Textos 📝 |
| `int` | Números inteiros 🔢 |
| `double` | Números com vírgula 📏 |
| `char` | Um único caractere 🔠 |
| `bool` | Verdadeiro ou falso ✅❌ |

Lembre-se:

- Toda variável tem um **tipo**.
- Toda variável tem um **nome**.
- Toda variável guarda um **valor**.

---

# Exercício 1 🧩

Crie as seguintes variáveis:

- Seu nome.
- Sua idade.
- Sua altura.
- Se você gosta de programar (`true` ou `false`).

Depois mostre todas usando `Console.WriteLine()`.

Exemplo de saída:

```
Nome: João
Idade: 18
Altura: 1.75
Gosta de programar: True
```

---

# Exercício 2 🧩

Crie uma variável para cada informação abaixo:

- Nome de um jogo (`string`)
- Quantidade de vidas (`int`)
- Velocidade do personagem (`double`)
- Primeira letra do nome do personagem (`char`)
- O jogo está pausado? (`bool`)

Depois imprima todas as informações na tela.

---

# Desafio Extra ⭐

Crie uma variável chamada `pontos` com o valor `100`.

Depois altere o valor para `250` e mostre o resultado.

Saída esperada:

```
250
```

💡 Esse desafio serve para mostrar que uma variável pode mudar de valor durante a execução do programa.