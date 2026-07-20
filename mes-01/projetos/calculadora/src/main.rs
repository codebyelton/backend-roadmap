use std::io;

fn main() {
    println!("Bem-vindo à calculadora!");

    loop {
        //Variáveis
        let mut entradaUser = String::new();

        //Menu
        println!("");
        println!("=== CALCULADORA ===");
        println!("1 - Somar");
        println!("2 - Subtrair");
        println!("3 - Multiplicar");
        println!("4 - Dividir");
        println!("5 - Sair");

        //Receber entrada do usuário
        println!("");
        println!("Escolha uma opção:");
        io::stdin().read_line(&mut entradaUser);
        let entradaUser: u32 = entradaUser.trim().parse().unwrap();
        println!("");
        println!("Você escolheu: {}", entradaUser);
        println!("");

        //Verificar escolha do usuário
        match entradaUser {
            1 => {
                println!("Você escolheu soma");
                println!("");
                println!("Digite o primeiro número:");
                let num1: u32 = recebeNumeroUser();

                println!("");
                println!("Digite o primeiro número:");
                let num2: u32 = recebeNumeroUser();

                let resultado = soma(num1, num2);
                println!("");
                println!("O resultado da soma foi: {}", resultado);
            }
            2 => {
                println!("Você escolheu subtração");
                println!("");
                println!("Digite o primeiro número:");
                let num1: u32 = recebeNumeroUser();

                println!("");
                println!("Digite o primeiro número:");
                let num2: u32 = recebeNumeroUser();

                let resultado = subtracao(num1, num2);
                println!("");
                println!("O resultado da subtração foi: {}", resultado);
            }
            3 => {
                println!("Você escolheu multiplicação");
                println!("");
                println!("Digite o primeiro número:");
                let num1: u32 = recebeNumeroUser();

                println!("");
                println!("Digite o primeiro número:");
                let num2: u32 = recebeNumeroUser();

                let resultado = multiplicacao(num1, num2);
                println!("");
                println!("O resultado da multiplicação foi: {}", resultado);
            }
            4 => {
                println!("Você escolheu divisão");
                println!("");
                println!("Digite o primeiro número:");
                let num1: u32 = recebeNumeroUser();

                println!("");
                println!("Digite o primeiro número:");
                let num2: u32 = recebeNumeroUser();

                if (num2 == 0) {
                    println!("Não é possível dividir por zero!");
                } else {
                    let resultado = divisao(num1, num2);
                    println!("");
                    println!("O resultado da divisão foi: {}", resultado);
                }
            }
            5 => {
                println!("Você escolheu sair");
                break;
            }
            _ => println!("Entrada Inválida"),
        }
    }
}

fn recebeNumeroUser() -> u32 {
    let mut entradaNumUser = String::new();

    io::stdin().read_line(&mut entradaNumUser);
    let entradaNumUser: u32 = entradaNumUser.trim().parse().unwrap();
    entradaNumUser
}

fn soma(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

fn subtracao(num1: u32, num2: u32) -> u32 {
    num1 - num2
}

fn multiplicacao(num1: u32, num2: u32) -> u32 {
    num1 * num2
}

fn divisao(num1: u32, num2: u32) -> f64 {
    num1 as f64 / num2 as f64
}
