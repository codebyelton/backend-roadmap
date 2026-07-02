use std::io;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

fn limpar_tela() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn main() {

    // ->>> Loop 1
    loop {
            // Número que o jogador precisa descobrir.
    let numero_secreto = 37;
    let mut numero_tentativas = 0;
        // Cria uma String vazia para armazenar a entrada do usuário.
        let mut numero_usuario = String::new();

        loop {

            numero_usuario.clear();

            // Exibe a mensagem para o jogador.
            println!("Tente descobrir o número entre 1 e 100");
            println!();

            // Lê o que o usuário digitou e armazena na String.
            io::stdin()
                .read_line(&mut numero_usuario)
                .expect("Falha ao ler a linha");

            // Tenta converter a String para um número (u32).
            // Se der certo, guarda o número.
            // Se der errado, mostra uma mensagem e volta para o início do loop.
            let numero_usuario: u32 = match numero_usuario.trim().parse() {
                // Conversão realizada com sucesso.
                Ok(numero) => numero,

                // Conversão falhou (o usuário digitou algo que não é um número).
                Err(_) => {
                    println!();
                    println!("Digite um número válido!");
                    println!();
                    continue;
                }
            };

            if numero_usuario == 0 || numero_usuario > 100 {
                println!();
                println!("Digite um número entre 1 e 100!");
                println!();
                continue;
            }

            numero_tentativas += 1;

            let distancia = numero_usuario.abs_diff(numero_secreto);

            // Verifica se o número digitado é igual ao número secreto.
            if numero_usuario == numero_secreto {
                // Se acertou, exibe a mensagem de vitória...
                println!();
                println!("Você acertou em {} tentativas!", numero_tentativas);
                println!();
                println!("Deseja jogar novamente? (s/n)");

                loop {
                    numero_tentativas = 0;

                    let mut reiniciar = String::new();

                    io::stdin()
                        .read_line(&mut reiniciar)
                        .expect("Falha ao ler a linha");

                    let reiniciar = reiniciar.trim();

                    match reiniciar {
                        "s" | "sim" => {
                            limpar_tela();
                            break;
                        }
                        "n" | "nao" | "não" => return,
                        _ => {
                            println!("Digite 's' ou 'n'.");
                            continue;
                        }
                    }
                }
            } else if distancia <= 5 {
                println!();
                println!("Muito perto!");
                println!();
            } else if distancia <= 10 {
                println!();
                println!("Perto!");
                println!();
            } else {
                println!();
                println!("Está longe!");
                println!();
            }
        }
    }
}
