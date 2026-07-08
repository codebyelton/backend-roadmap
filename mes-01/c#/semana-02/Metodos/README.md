# Métodos em C# 🛠️

Imagine que um **método** é como um botão de um controle remoto. 📺

Quando você aperta o botão, alguma coisa acontece.

No C#, é a mesma ideia.

Você cria um método e, quando quiser executar aquela tarefa, basta chamá-lo.

Assim você evita escrever o mesmo código várias vezes. 😄

---

# O que é um método?

Um método é um bloco de código que faz uma tarefa específica.

Exemplo:

```csharp
using System;

class Program
{
    static void Main()
    {
        Saudacao();
    }

    static void Saudacao()
    {
        Console.WriteLine("Olá!");
    }
}
```

### Resultado

```
Olá!
```

Perceba que o código dentro de `Saudacao()` só foi executado porque chamamos o método dentro do `Main()`.

---

# Criando um método

A estrutura de um método é esta:

```csharp
static void NomeDoMetodo()
{
    // Código
}
```

Exemplo:

```csharp
static void MostrarMensagem()
{
    Console.WriteLine("Bem-vindo!");
}
```

Depois, para executar o método:

```csharp
MostrarMensagem();
```

---

# Chamando um método

Você pode chamar um método quantas vezes quiser.

```csharp
using System;

class Program
{
    static void Main()
    {
        Saudacao();
        Saudacao();
        Saudacao();
    }

    static void Saudacao()
    {
        Console.WriteLine("Olá!");
    }
}
```

### Resultado

```
Olá!
Olá!
Olá!
```

Isso evita repetir o mesmo código várias vezes.

---

# Métodos com parâmetros 📦

Às vezes queremos que o método receba uma informação.

Essa informação é chamada de **parâmetro**.

Exemplo:

```csharp
using System;

class Program
{
    static void Main()
    {
        Saudacao("Maria");
    }

    static void Saudacao(string nome)
    {
        Console.WriteLine($"Olá, {nome}!");
    }
}
```

### Resultado

```
Olá, Maria!
```

Agora o método funciona com qualquer nome.

```csharp
Saudacao("João");
Saudacao("Ana");
Saudacao("Pedro");
```

---

# Métodos com mais de um parâmetro

Um método pode receber várias informações.

```csharp
using System;

class Program
{
    static void Main()
    {
        MostrarPessoa("Carlos", 25);
    }

    static void MostrarPessoa(string nome, int idade)
    {
        Console.WriteLine($"{nome} tem {idade} anos.");
    }
}
```

### Resultado

```
Carlos tem 25 anos.
```

---

# Métodos que retornam valores 🎁

Alguns métodos apenas executam uma tarefa.

Outros fazem um cálculo e devolvem um resultado.

Para isso usamos o `return`.

Exemplo:

```csharp
using System;

class Program
{
    static void Main()
    {
        int resultado = Somar(5, 3);

        Console.WriteLine(resultado);
    }

    static int Somar(int numero1, int numero2)
    {
        return numero1 + numero2;
    }
}
```

### Resultado

```
8
```

Nesse caso, o método fez a conta e devolveu o resultado.

---

# O que significa "void"?

Quando um método é `void`, significa que ele **não devolve nenhum valor**.

Ele apenas executa uma tarefa.

Exemplo:

```csharp
static void Mensagem()
{
    Console.WriteLine("Olá!");
}
```

Já quando ele devolve um valor, usamos outro tipo.

Exemplo:

```csharp
static int Somar()
{
    return 10;
}
```

---

# Boas práticas ✨

Escolha nomes que expliquem o que o método faz.

✅ Bons exemplos

```csharp
CalcularMedia();
MostrarMenu();
CadastrarUsuario();
Somar();
```

❌ Evite nomes como

```csharp
Teste();
Metodo1();
Coisa();
```

Quanto mais claro o nome, mais fácil será entender o código.

---

# Resumo Rápido 📝

- Um método é um bloco de código que faz uma tarefa.
- Você pode chamar um método quantas vezes quiser.
- Parâmetros servem para enviar informações ao método.
- Um método pode receber vários parâmetros.
- `void` significa que o método não retorna nada.
- `return` devolve um valor para quem chamou o método.

---

# Exercício 1 🧩

Crie um método chamado:

```csharp
BoasVindas()
```

Ele deve mostrar:

```
Bem-vindo ao curso de C#!
```

Depois chame esse método dentro do `Main()`.

---

# Exercício 2 🧩

Crie um método chamado:

```csharp
MostrarNome(string nome)
```

Ele deve mostrar:

```
Olá, João!
```

Troque o nome por qualquer outro e veja o resultado.

---

# Desafio Extra ⭐

Crie um método chamado:

```csharp
Multiplicar(int numero1, int numero2)
```

Ele deve retornar o resultado da multiplicação usando `return`.

Depois, no `Main()`, guarde o resultado em uma variável e mostre na tela.

Exemplo:

```text
Resultado: 20
```

💡 Esse desafio junta tudo o que você aprendeu: criar métodos, usar parâmetros e retornar valores.