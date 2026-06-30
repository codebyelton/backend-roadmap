use std::io;

fn main() {

    // Exercício 1: Recebendo nome do usuário.
    let mut nome = String::new();

    println!("Qual é o seu nome?");

    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler a linha");

    println!("Olá, {}!", nome.trim());

    // Exercício 2: Recebendo idade do usuário.
let mut idade = String::new();

println!("Qual é a sua idade?");

io::stdin()
    .read_line(&mut idade)
    .expect("Falha ao ler a linha");

let idade: u32 = idade
    .trim()
    .parse()
    .expect("Digite um número válido");

println!("Você tem {} anos.", idade);

// Exercício 3: Recebendo nome e idade do usuário

let mut nome1 = String::new();
let mut idade1 = String::new();

println!("Qual é o seu nome?");

io::stdin()
    .read_line(&mut nome1)
    .expect("Falha ao ler a linha");

println!("Qual é a sua idade?");

io::stdin()
    .read_line(&mut idade1)
    .expect("Falha ao ler a linha");

let idade1: u32 = idade1
    .trim()
    .parse()
    .expect("Digite um número válido");

    println!("Olá, {}! Você tem {} anos.", nome1.trim(), idade);


}
