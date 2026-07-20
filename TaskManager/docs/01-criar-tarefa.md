# 📝 TaskManager — Criar Tarefa

> 🎯 **Objetivo:** criar a primeira funcionalidade do seu TaskManager: permitir que o usuário cadastre uma nova tarefa.

---

## 🧠 O que você precisa construir?

Seu programa deverá permitir que o usuário informe os dados de uma tarefa.

Por enquanto, pense em uma tarefa simples. Ela pode ter, por exemplo:

* Um título
* Uma descrição
* Uma data de criação

💡 **Não precisa criar um sistema complexo agora.** O objetivo desta etapa é praticar a criação de objetos e a entrada de dados pelo usuário.

---

# 🏗️ Etapa 1 — Pense no objeto

Antes de escrever código, faça esta pergunta:

> ❓ O que é uma tarefa dentro do meu sistema?

Uma tarefa é algo que possui **características**.

Por exemplo:

```text
Tarefa
├── Título
├── Descrição
└── Data de criação
```

Agora pense:

> 🤔 Qual tipo de dado combina com cada característica?

Dicas:

* Para textos, você provavelmente precisará de um tipo para representar palavras e frases.
* Para datas, existe um tipo específico no C#.
* O título e a descrição são obrigatórios ou podem ficar vazios?

💡 **Tente criar uma classe que represente uma tarefa.**

---

# 🧱 Etapa 2 — Crie a classe da tarefa

Agora crie uma classe responsável por representar uma tarefa.

Pense:

> ❓ Onde essa classe deveria ficar no seu projeto?

Uma sugestão de organização:

```text
TaskManager/
│
├── Program.cs
│
└── Models/
    └── Tarefa.cs
```

📌 A pasta `Models` pode conter as classes que representam os objetos principais do seu sistema.

---

# 🧩 Etapa 3 — Crie as propriedades

Sua classe precisa armazenar os dados da tarefa.

Antes de escrever, responda:

* Qual será o nome da propriedade que guarda o título?
* Qual será o nome da propriedade que guarda a descrição?
* Qual será o nome da propriedade que guarda a data?

💡 Pense também:

> ❓ Uma propriedade de texto deve permitir `null`?

> ❓ Uma tarefa deve ser criada sem título?

Você não precisa resolver tudo de forma perfeita agora. O objetivo é começar a pensar nas regras do seu sistema.

---

# ⌨️ Etapa 4 — Receba os dados do usuário

Agora você precisa permitir que o usuário digite os dados da tarefa.

Pense no fluxo:

```text
Programa inicia
      ↓
Usuário informa o título
      ↓
Usuário informa a descrição
      ↓
O sistema cria uma tarefa
```

💡 Dica:

Você precisará pesquisar como:

* Ler informações digitadas no console.
* Armazenar essas informações em variáveis.
* Utilizar essas informações para criar um objeto.

⚠️ **Não crie a tarefa diretamente com valores fixos.**

Evite algo como:

```text
Título = "Estudar C#"
```

O objetivo é que o usuário possa criar diferentes tarefas.

---

# 🏭 Etapa 5 — Crie o objeto

Agora que você possui os dados digitados pelo usuário, chegou a hora de criar uma instância da sua classe.

Pense:

> ❓ Como eu crio um objeto a partir de uma classe?

Você já estudou ou estudará conceitos importantes de:

* Classes
* Objetos
* Propriedades
* Instanciação

💡 Tente criar um objeto `Tarefa` utilizando os dados que o usuário informou.

---

# 🕒 Etapa 6 — Data de criação

Toda tarefa deveria saber quando foi criada.

Agora pense:

> ❓ A data de criação precisa ser digitada pelo usuário?

Provavelmente não.

O próprio programa pode descobrir a data atual.

💡 Pesquise:

```text
Como obter a data e hora atual em C#
```

Depois, utilize essa informação ao criar a tarefa.

---

# 🖥️ Etapa 7 — Mostre o resultado

Depois de criar a tarefa, mostre uma confirmação para o usuário.

Algo parecido com:

```text
✅ Tarefa criada com sucesso!

Título: [título informado]
Descrição: [descrição informada]
Data de criação: [data]
```

⚠️ Você deve construir essa saída sozinho.

Pense:

> ❓ Como acessar uma propriedade de um objeto?

---

# 🧪 Desafio extra — Validação

Depois que sua primeira versão estiver funcionando, tente melhorar.

O que deveria acontecer se o usuário não digitar um título?

Por exemplo:

```text
Título:
```

Pense em uma regra:

```text
Se o título estiver vazio
    mostrar uma mensagem de erro
Caso contrário
    criar a tarefa
```

💡 Pesquise como verificar se uma string está vazia ou contém apenas espaços.

---

# ✅ Checklist

Antes de considerar esta etapa concluída, verifique:

* [ ] Criei uma classe para representar uma tarefa.
* [ ] Criei propriedades para os dados da tarefa.
* [ ] Consigo receber informações digitadas pelo usuário.
* [ ] Consigo criar um objeto `Tarefa`.
* [ ] A tarefa possui uma data de criação.
* [ ] Consigo exibir a tarefa criada no console.
* [ ] Adicionei alguma validação para o título.
* [ ] Organizei meus arquivos de forma clara.
* [ ] Entendi o que escrevi, sem apenas copiar código.

---

# 🚫 Regras deste exercício

Durante este exercício, tente evitar:

* ❌ Procurar a solução completa do TaskManager.
* ❌ Copiar um projeto pronto.
* ❌ Criar funcionalidades que ainda não foram solicitadas.
* ❌ Usar banco de dados neste momento.
* ❌ Criar uma API neste momento.
* ❌ Criar uma arquitetura complexa.

🎯 **Seu objetivo agora é apenas: criar uma tarefa.**

---

# 🧠 Perguntas para refletir

Depois de terminar, tente responder:

1. O que é uma classe?
2. O que é um objeto?
3. Por que a tarefa possui propriedades?
4. Qual é a diferença entre uma classe `Tarefa` e um objeto criado a partir dela?
5. Onde acontece a criação da tarefa?
6. O que aconteceria se você precisasse criar 10 tarefas?
7. Onde essas tarefas estão sendo armazenadas atualmente?

💡 A última pergunta será muito importante para a próxima etapa: **Listar Tarefas**.
