# Mini RPG Attack System 🗡️

Projeto simples em Rust para praticar o uso de `match`.

## 📚 Conceitos praticados

* Variáveis
* Tipos (`&str`)
* Estrutura `match`
* Pattern Matching
* Saída no terminal com `println!`

## ⚔️ Ataques disponíveis

| Ataque | Dano |
| ------ | ---: |
| Espada |   10 |
| Arco   |    7 |
| Magia  |   15 |

Qualquer valor diferente dos ataques acima resulta em:

```text
Ataque Inválido
```

## 🚀 Executando

```bash
cargo run
```

## 💻 Exemplo

Com:

```rust
let ataque = "magia";
```

Saída:

```text
Dano: 15
```

## 🎯 Objetivo

Este projeto foi desenvolvido como parte dos meus estudos em Rust para praticar a estrutura `match` e a lógica de seleção de casos.
