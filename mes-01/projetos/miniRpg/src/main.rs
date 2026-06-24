fn main() {
    
    //Variáveis
    let ataque: &str = "magia";
    
    //Match
    match ataque {
        "espada" => println!("Dano: 10"),
        "arco" => println!("Dano: 7"),
        "magia" => println!("Dano: 15"),
        _ => println!("Ataque Inválido"),
    }
}