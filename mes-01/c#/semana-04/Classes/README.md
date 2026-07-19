# Classes em C# 🏠

Imagine que você quer criar um personagem para um jogo. 🎮

Esse personagem pode ter:

- Nome 📝
- Vida ❤️
- Velocidade 🏃
- Poderes ⚡

Uma **classe** é como uma planta ou um molde que explica como esse personagem será.

A classe diz:

> "Todo personagem terá essas informações e poderá fazer essas coisas."

---

# Criando uma classe

```csharp
class Personagem
{
    public string nome;
    public int vida;
}
```

Criamos uma classe chamada `Personagem`.

Ela possui duas informações:

```csharp
public string nome;
public int vida;
```

Essas informações são chamadas de **campos**.

---

# Criando um objeto 🧱

A classe é o molde.

O objeto é algo criado usando esse molde.

```csharp
Personagem jogador = new Personagem();
```

Aqui criamos um objeto chamado `jogador`.

Agora podemos preencher suas informações:

```csharp
jogador.nome = "Guerreiro";
jogador.vida = 100;
```

E podemos mostrar os valores:

```csharp
Console.WriteLine(jogador.nome);
Console.WriteLine(jogador.vida);
```

Resultado:

```text
Guerreiro
100
```

---

# Classe e objeto 🧠

Imagine uma fábrica de brinquedos. 🧸

### Classe

É o molde:

```text
Personagem
```

### Objeto

É o brinquedo criado usando o molde:

```text
jogador
```

Você pode criar vários objetos usando a mesma classe:

```csharp
Personagem jogador1 = new Personagem();
Personagem jogador2 = new Personagem();
```

Os dois são `Personagem`, mas podem ter valores diferentes.

---

# Métodos dentro de uma classe ⚙️

Uma classe também pode ter métodos.

Assim, o personagem pode fazer coisas.

```csharp
class Personagem
{
    public string nome;

    public void Atacar()
    {
        Console.WriteLine($"{nome} atacou!");
    }
}
```

Agora podemos fazer:

```csharp
Personagem jogador = new Personagem();

jogador.nome = "Guerreiro";

jogador.Atacar();
```

Resultado:

```text
Guerreiro atacou!
```

A classe agora possui:

- Dados → `nome`
- Ações → `Atacar()`

---

# Campos e métodos

Uma classe pode guardar:

## Dados 📦

```csharp
public string nome;
public int vida;
```

Esses são os **campos**.

## Ações ⚙️

```csharp
public void Atacar()
{
    Console.WriteLine("Atacando!");
}
```

Esse é um **método**.

Uma classe pode ter vários campos e métodos.

---

# Construtor 🏗️

O construtor é executado automaticamente quando criamos um objeto.

Ele possui o mesmo nome da classe.

```csharp
class Personagem
{
    public string nome;

    public Personagem()
    {
        nome = "Sem nome";
    }
}
```

Quando fazemos:

```csharp
Personagem jogador = new Personagem();
```

O construtor é executado automaticamente.

---

# Construtor com parâmetros

Podemos enviar informações para o construtor.

```csharp
class Personagem
{
    public string nome;
    public int vida;

    public Personagem(string nome, int vida)
    {
        this.nome = nome;
        this.vida = vida;
    }
}
```

Agora podemos criar um personagem já preenchido:

```csharp
Personagem jogador = new Personagem("Guerreiro", 100);
```

Agora:

```text
Nome: Guerreiro
Vida: 100
```

---

# O que significa public? 🔓

`public` significa que algo pode ser acessado de fora da classe.

Exemplo:

```csharp
public string nome;
```

Podemos fazer:

```csharp
jogador.nome = "Guerreiro";
```

Mais para frente você aprenderá sobre `private`, que serve para proteger informações dentro da classe. 🔒

---

# Um exemplo completo 🎮

```csharp
using System;

class Personagem
{
    public string nome;
    public int vida;

    public void MostrarInformacoes()
    {
        Console.WriteLine($"Nome: {nome}");
        Console.WriteLine($"Vida: {vida}");
    }
}

class Program
{
    static void Main()
    {
        Personagem jogador = new Personagem();

        jogador.nome = "Guerreiro";
        jogador.vida = 100;

        jogador.MostrarInformacoes();
    }
}
```

Resultado:

```text
Nome: Guerreiro
Vida: 100
```

---

# Resumo Rápido 📝

- Classe é um molde ou modelo.
- Objeto é algo criado usando uma classe.
- `new` cria um novo objeto.
- Campos guardam informações.
- Métodos representam ações.
- Construtores ajudam a criar objetos já configurados.
- Uma classe pode criar vários objetos.
- Cada objeto pode ter seus próprios valores.

---

# Exercício 1 🧩

Crie uma classe chamada:

```csharp
class Cachorro
```

Ela deve possuir:

```csharp
public string nome;
public int idade;
```

Depois crie um objeto:

```csharp
Cachorro cachorro = new Cachorro();
```

Preencha o nome e a idade e mostre na tela.

---

# Exercício 2 🧩

Adicione um método chamado:

```csharp
Latir()
```

Ele deve mostrar:

```text
Au Au! 🐶
```

Depois crie um objeto `Cachorro` e chame o método.

---

# Desafio Extra ⭐

Crie uma classe chamada:

```csharp
Personagem
```

Ela deve possuir:

- `nome`
- `vida`
- `nivel`

E também um método:

```csharp
MostrarStatus()
```

Que mostre:

```text
Nome: Guerreiro
Vida: 100
Nível: 1
```

💡 Esse é um dos primeiros passos para entender **Programação Orientada a Objetos (POO)**. Classes são usadas para organizar dados e ações que pertencem à mesma coisa.