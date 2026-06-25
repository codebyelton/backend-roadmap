fn main() {
    
    // Exercício 1 - Uso de If e Else
    
    let idade = 15;
    
    if idade >= 18 {println!("Pode Entrar")} else {println!("Não pode entrar")}
    
    // Exercício 2 - Uso de match
    
    let animal = "gato";
    
    match animal {
        "gato" => println!("Miau"),
        "cachorro" => println!("Au Au"),
        "Passarinho" => println!("Piu Piu"),
        _ => println!("Animal desconhecido"),
    }
    
}