use std::io;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

// Limpa todo o conteúdo visível do terminal.
fn limpar_tela() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn main() {

    // Loop principal responsável por reiniciar uma nova partida.
    loop {
        let numero_secreto = 37;
        let mut numero_tentativas = 0;

        // Buffer reutilizado para armazenar a entrada do usuário.
        let mut numero_usuario = String::new();

        // Loop responsável pela lógica da partida atual.
        loop {

            // Reutiliza a mesma String sem realizar nova alocação.
            numero_usuario.clear();

            println!("Tente descobrir o número entre 1 e 100");
            println!();

            io::stdin()
                .read_line(&mut numero_usuario)
                .expect("Falha ao ler a linha");

            // Converte a entrada para u32 ou reinicia a iteração em caso de erro.
            let numero_usuario: u32 = match numero_usuario.trim().parse() {
                Ok(numero) => numero,

                Err(_) => {
                    println!();
                    println!("Digite um número válido!");
                    println!();
                    continue;
                }
            };

            // Garante que o valor esteja dentro do intervalo permitido.
            if numero_usuario == 0 || numero_usuario > 100 {
                println!();
                println!("Digite um número entre 1 e 100!");
                println!();
                continue;
            }

            numero_tentativas += 1;

            // Calcula a diferença absoluta entre a tentativa e o número secreto.
            let distancia = numero_usuario.abs_diff(numero_secreto);

            if numero_usuario == numero_secreto {
                println!();
                println!("Você acertou em {} tentativas!", numero_tentativas);
                println!();
                println!("Deseja jogar novamente? (s/n)");

                // Aguarda uma resposta válida para reiniciar ou encerrar o jogo.
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

                            // Encerra apenas o loop de confirmação.
                            break;
                        }
                        "n" | "nao" | "não" => return,

                        // Mantém o loop até que uma entrada válida seja informada.
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