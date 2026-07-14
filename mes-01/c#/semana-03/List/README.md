# Lists em C# 📋

Imagine uma mochila. 🎒

Você pode colocar quantas coisas quiser.

Se precisar, pode tirar alguma coisa depois.

A **List** funciona exatamente assim.

Ela é parecida com um Array, mas pode crescer e diminuir quando você quiser.

---

# Importando a List

Antes de usar uma List, precisamos importar:

```csharp
using System.Collections.Generic;
```

---

# Criando uma List

```csharp
List<string> frutas = new List<string>();
```

Agora temos uma lista vazia.

---

# Adicionando elementos

Usamos o método `Add()`.

```csharp
frutas.Add("Maçã");
frutas.Add("Banana");
frutas.Add("Uva");
```

Agora a lista possui três itens.

---

# Acessando um elemento

Funciona igual ao Array.

```csharp
Console.WriteLine(frutas[0]);
```

Resultado

```
Maçã
```

---

# Alterando um elemento

Também funciona igual.

```csharp
frutas[1] = "Laranja";
```

Agora temos:

```
Maçã
Laranja
Uva
```

---

# Removendo elementos

Usamos `Remove()`.

```csharp
frutas.Remove("Banana");
```

Agora sobra:

```
Maçã
Uva
```

---

# Quantidade de elementos

Na List usamos:

```csharp
frutas.Count
```

Exemplo

```csharp
Console.WriteLine(frutas.Count);
```

Resultado

```
3
```

⚠️ Lembre-se:

- Array → `Length`
- List → `Count`

---

# Percorrendo uma List

Também usamos `foreach`.

```csharp
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

# Quando usar List?

Sempre que você precisar:

- Adicionar elementos.
- Remover elementos.
- Ter uma coleção que pode aumentar ou diminuir.

Na maioria dos projetos em C#, a **List** é mais usada que o Array.

---

# Resumo Rápido 📝

- List guarda vários elementos.
- Todos os elementos precisam ser do mesmo tipo.
- Pode aumentar e diminuir de tamanho.
- `Add()` adiciona elementos.
- `Remove()` remove elementos.
- `Count` mostra a quantidade de itens.
- `foreach` percorre todos os elementos.

---

# Exercício 1 🧩

Crie uma List chamada:

```csharp
List<string> nomes = new List<string>();
```

Depois:

- Adicione 3 nomes.
- Mostre todos usando `foreach`.

---

# Exercício 2 🧩

Crie uma List de números.

Adicione:

```
10
20
30
40
```

Remova o número `20`.

Depois mostre todos os números.

Resultado esperado

```
10
30
40
```

---

# Desafio Extra ⭐

Crie uma List de cidades.

Adicione cinco cidades usando `Add()`.

Depois:

- Mostre todas.
- Mostre a quantidade usando `Count`.

💡 Esse desafio pratica praticamente tudo que você aprendeu sobre **List**.