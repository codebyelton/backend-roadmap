# Revisão da Sintaxe em C# 📚

Antes de aprender coisas mais avançadas, é importante entender como um programa em C# é organizado.

Pense na sintaxe como as **regras do jogo**. 🎮

Se você seguir as regras, o programa funciona. Se não seguir, o compilador mostra um erro.

---

# Estrutura de um programa

Um programa simples em C# é parecido com isso:

```csharp
using System;

class Program
{
    static void Main()
    {
        Console.WriteLine("Olá, Mundo!");
    }
}
```

Parece muita coisa no começo, mas vamos por partes. 😊

---

# using System;

```csharp
using System;
```

Essa linha importa algumas ferramentas do C#.

Uma delas é o `Console`, que usamos para mostrar mensagens na tela.

Sem ela, o programa não saberia o que é `Console`.

---

# class Program

```csharp
class Program
{
}
```

Uma **classe** é como uma caixa onde colocamos nosso código.

Por enquanto, basta lembrar:

> Todo código ficará dentro de uma classe.

Mais para frente você vai aprender tudo sobre classes.

---

# Main()

```csharp
static void Main()
{
}
```

O método `Main()` é o ponto de partida do programa.

É por ele que o C# começa a executar o código.

Tudo que estiver dentro dele será executado.

---

# Console.WriteLine()

Serve para mostrar uma mensagem na tela.

```csharp
Console.WriteLine("Olá!");
```

Resultado:

```
Olá!
```

Também podemos mostrar variáveis.

```csharp
string nome = "Maria";

Console.WriteLine(nome);
```

Resultado:

```
Maria
```

---

# Ponto e vírgula (;)

No C#, quase toda instrução termina com **`;`**.

✅ Certo

```csharp
int idade = 20;
```

❌ Errado

```csharp
int idade = 20
```

Se esquecer o `;`, o compilador mostrará um erro.

---

# Chaves { }

As chaves servem para indicar onde um bloco de código começa e termina.

Exemplo:

```csharp
if (true)
{
    Console.WriteLine("Olá!");
}
```

Tudo que estiver dentro das chaves pertence ao `if`.

O mesmo acontece com métodos, classes, loops e várias outras estruturas.

---

# Parênteses ( )

Os parênteses aparecem em vários lugares.

Por exemplo:

```csharp
Console.WriteLine("Olá");
```

ou

```csharp
if (idade >= 18)
{
    Console.WriteLine("Maior de idade");
}
```

Eles normalmente recebem informações ou condições.

---

# Comentários 💬

Comentários servem para explicar o código.

Eles não são executados pelo programa.

Comentário de uma linha:

```csharp
// Esse é um comentário
```

Comentário de várias linhas:

```csharp
/*
Esse comentário
ocupa várias linhas.
*/
```

Use comentários para lembrar o que o código faz.

---

# Indentação 📏

Indentar significa organizar o código.

Veja um exemplo bonito:

```csharp
if (true)
{
    Console.WriteLine("Olá");
}
```

Agora um exemplo difícil de ler:

```csharp
if(true){
Console.WriteLine("Olá");
}
```

Os dois funcionam, mas o primeiro é muito mais fácil de entender.

---

# Letras maiúsculas e minúsculas

O C# diferencia letras maiúsculas e minúsculas.

Isso significa que:

```csharp
idade
```

é diferente de

```csharp
Idade
```

E também:

✅ Certo

```csharp
Console.WriteLine("Olá");
```

❌ Errado

```csharp
console.writeline("Olá");
```

O compilador entende que são coisas diferentes.

---

# Resumo Rápido 📝

- `using System;` importa ferramentas do C#.
- `class Program` guarda nosso código.
- `Main()` é onde o programa começa.
- `Console.WriteLine()` mostra mensagens.
- Quase toda linha termina com `;`.
- `{ }` delimitam blocos de código.
- `( )` são usados em métodos e condições.
- Comentários ajudam a explicar o código.
- Indente seu código para deixá-lo organizado.
- C# diferencia letras maiúsculas e minúsculas.

---

# Exercício 1 🧩

Escreva um programa que mostre na tela:

```
Olá!
Meu nome é João.
Estou aprendendo C#.
```

Use um `Console.WriteLine()` para cada frase.

---

# Exercício 2 🧩

Copie o código abaixo e encontre os erros.

```csharp
using System

class Program
{
    static void Main()
    {
        Console.Writeline("Olá")
    }
}
```

Depois corrija o código para que ele funcione.

---

# Desafio Extra ⭐

Crie um programa que:

- Tenha pelo menos uma variável.
- Mostre essa variável usando `Console.WriteLine()`.
- Adicione dois comentários explicando o que o programa faz.

💡 O objetivo é praticar a estrutura completa de um programa em C# e começar a escrever códigos organizados.