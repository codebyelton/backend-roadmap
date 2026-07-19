# Encapsulamento em C# 🔒

Imagine que você tem um cofrinho. 🐷💰

Você coloca dinheiro dentro dele, mas não quer que qualquer pessoa possa abrir e pegar o dinheiro quando quiser.

Então você protege o cofrinho.

O **encapsulamento** funciona de forma parecida.

Ele serve para **proteger os dados de uma classe** e controlar como eles podem ser acessados ou alterados.

---

# O problema de deixar tudo público ⚠️

Imagine esta classe:

```csharp
class Personagem
{
    public int vida;
}
```

Agora qualquer pessoa pode fazer:

```csharp
Personagem jogador = new Personagem();

jogador.vida = -500;
```

Isso não faz muito sentido. 😅

Um personagem não deveria ter:

```text
Vida: -500
```

O problema é que, como `vida` é `public`, qualquer código pode alterar seu valor livremente.

---

# Usando private 🔒

Para proteger uma informação, podemos usar `private`.

```csharp
class Personagem
{
    private int vida;
}
```

Agora a variável `vida` só pode ser acessada dentro da própria classe.

Isso não funcionará:

```csharp
Personagem jogador = new Personagem();

jogador.vida = 100;
```

Porque `vida` está protegida.

---

# Como alterar uma informação protegida?

Podemos criar métodos para controlar o acesso.

```csharp
class Personagem
{
    private int vida;

    public void ReceberDano(int dano)
    {
        vida -= dano;
    }
}
```

Agora o personagem pode receber dano através de um método.

```csharp
Personagem jogador = new Personagem();

jogador.ReceberDano(20);
```

O código não altera a vida diretamente.

Ele pede para a classe fazer isso.

---

# Controlando os valores 🛡️

Podemos colocar regras dentro dos métodos.

```csharp
class Personagem
{
    private int vida = 100;

    public void ReceberDano(int dano)
    {
        if (dano > 0)
        {
            vida -= dano;
        }
    }
}
```

Agora o personagem só recebe dano se o valor for maior que zero.

Isso evita situações como:

```csharp
jogador.ReceberDano(-50);
```

---

# Propriedades (Properties) 📦

Em C#, uma forma muito comum de usar encapsulamento é através de propriedades.

Exemplo:

```csharp
class Personagem
{
    public string Nome { get; set; }
}
```

Aqui temos:

- `get` → permite ler o valor.
- `set` → permite alterar o valor.

Podemos fazer:

```csharp
Personagem jogador = new Personagem();

jogador.Nome = "Guerreiro";

Console.WriteLine(jogador.Nome);
```

---

# Somente leitura 👀

Podemos permitir que o valor seja lido, mas não alterado de fora da classe.

```csharp
class Personagem
{
    public int Vida { get; private set; }

    public Personagem()
    {
        Vida = 100;
    }
}
```

Agora podemos fazer:

```csharp
Console.WriteLine(jogador.Vida);
```

Mas isso não:

```csharp
jogador.Vida = 500;
```

A vida só pode ser alterada dentro da própria classe.

🔒 Muito mais seguro!

---

# Exemplo completo 🎮

```csharp
class Personagem
{
    public string Nome { get; private set; }

    public int Vida { get; private set; }

    public Personagem(string nome)
    {
        Nome = nome;
        Vida = 100;
    }

    public void ReceberDano(int dano)
    {
        if (dano > 0)
        {
            Vida -= dano;
        }
    }
}
```

Usando a classe:

```csharp
Personagem jogador = new Personagem("Guerreiro");

Console.WriteLine(jogador.Nome);
Console.WriteLine(jogador.Vida);

jogador.ReceberDano(20);

Console.WriteLine(jogador.Vida);
```

Resultado:

```text
Guerreiro
100
80
```

O código de fora não pode simplesmente fazer:

```csharp
jogador.Vida = -500;
```

A classe controla como a vida pode mudar.

---

# Por que usar encapsulamento? 🤔

O encapsulamento ajuda a:

- 🔒 Proteger informações.
- 🚫 Evitar valores inválidos.
- 🧹 Organizar melhor o código.
- 🛠️ Controlar como os dados são alterados.
- 🐛 Evitar erros no programa.

Pense assim:

> Os dados importantes ficam protegidos e a classe decide como eles podem ser usados.

---

# Resumo Rápido 📝

- Encapsulamento é proteger os dados de uma classe.
- `private` impede o acesso direto de fora.
- `public` permite acesso de fora.
- Métodos podem controlar como os dados são alterados.
- `get` permite ler uma propriedade.
- `set` permite alterar uma propriedade.
- `private set` permite alterar o valor apenas dentro da própria classe.

---

# Exercício 1 🧩

Crie uma classe chamada:

```csharp
Conta
```

Ela deve possuir:

```csharp
private double saldo;
```

Depois crie um método:

```csharp
Depositar(double valor)
```

Esse método deve adicionar dinheiro ao saldo.

Crie também:

```csharp
MostrarSaldo()
```

Que mostre o saldo atual.

---

# Exercício 2 🧩

Crie uma classe chamada:

```csharp
Personagem
```

Ela deve ter:

```csharp
public int Vida { get; private set; }
```

Comece com:

```text
100
```

Depois crie um método:

```csharp
ReceberDano(int dano)
```

O método deve diminuir a vida.

💡 A vida não deve poder ser alterada diretamente de fora da classe.

---

# Desafio Extra ⭐

Crie uma classe chamada:

```csharp
Cofrinho
```

Ela deve possuir:

```csharp
private double dinheiro;
```

Crie métodos para:

- Adicionar dinheiro.
- Retirar dinheiro.
- Mostrar o saldo.

Mas crie uma regra:

> Não pode retirar mais dinheiro do que existe no cofrinho. 🐷💰

💡 Esse é o objetivo do encapsulamento: **proteger os dados e controlar as regras de como eles podem ser usados**.