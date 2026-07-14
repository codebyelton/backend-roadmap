# Arrays em C# 📦

Imagine que um **Array** é como uma cartela de ovos. 🥚

Ela tem um número fixo de espaços.

Você pode colocar um ovo em cada espaço, mas **não consegue aumentar a cartela depois que ela foi criada**.

Em C#, o Array funciona do mesmo jeito.

---

# O que é um Array?

Um Array guarda vários valores do mesmo tipo.

Exemplo:

```csharp
string[] frutas = { "Maçã", "Banana", "Uva" };
```

Aqui temos um Array com três frutas.

Todos os elementos precisam ser do mesmo tipo.

✅ Certo

```csharp
int[] numeros = { 10, 20, 30 };
```

❌ Errado

```csharp
int[] numeros = { 10, "João", true };
```

---

# Índices

Cada elemento possui uma posição chamada **índice**.

⚠️ O índice sempre começa em **0**.

| Índice | Valor |
|--------|--------|
| 0 | Maçã |
| 1 | Banana |
| 2 | Uva |

---

# Acessando um elemento

Basta informar o índice.

```csharp
Console.WriteLine(frutas[0]);
```

Resultado

```
Maçã
```

Outro exemplo

```csharp
Console.WriteLine(frutas[2]);
```

Resultado

```
Uva
```

---

# Alterando um valor

Você pode trocar um elemento.

```csharp
string[] frutas = { "Maçã", "Banana", "Uva" };

frutas[1] = "Laranja";
```

Agora o Array ficou:

```
Maçã
Laranja
Uva
```

---

# Descobrindo o tamanho

Usamos:

```csharp
frutas.Length
```

Exemplo

```csharp
Console.WriteLine(frutas.Length);
```

Resultado

```
3
```

---

# Percorrendo um Array

O jeito mais fácil é usando `foreach`.

```csharp
string[] frutas = { "Maçã", "Banana", "Uva" };

foreach (string fruta in frutas)
{
    Console.WriteLine(fruta);
}
```

Resultado

```
Maçã
Banana
Uva
```

---

# Limitação do Array

Depois de criado, o Array **não aumenta nem diminui**.

Exemplo

```csharp
int[] numeros = { 10, 20, 30 };
```

Ele sempre terá apenas 3 posições.

Se precisar adicionar novos itens depois, use uma **List**.

---

# Resumo Rápido 📝

- Array guarda vários valores.
- Todos os elementos possuem o mesmo tipo.
- O índice começa em **0**.
- O tamanho é fixo.
- Usamos `Length` para saber a quantidade de elementos.
- Podemos percorrer usando `foreach`.

---

# Exercício 1 🧩

Crie um Array chamado:

```csharp
string[] animais = { "Cachorro", "Gato", "Coelho" };
```

Mostre:

- O primeiro animal.
- O último animal.

Resultado esperado

```
Cachorro
Coelho
```

---

# Exercício 2 🧩

Crie um Array de números.

```csharp
int[] numeros = { 5, 10, 15, 20 };
```

Use um `foreach` para mostrar todos os números.

Resultado esperado

```
5
10
15
20
```

---

# Desafio Extra ⭐

Crie um Array com os meses:

```
Janeiro
Fevereiro
Março
Abril
```

Depois mostre:

- O segundo mês.
- A quantidade de meses usando `Length`.