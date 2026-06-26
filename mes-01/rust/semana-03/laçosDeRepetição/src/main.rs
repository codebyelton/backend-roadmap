fn main() {
    
    // Exercício 1 - Loop
    
    let mut contador = 1;
    
    loop {
        println!("{}", contador);
        
        contador += 1;
        
        if contador > 5 {break;}
    }
    
    println!("============================================");
    
    // Exercício 2 - While
    
    let mut energia = 5;
    
    while energia > 0 {
        println!("{}", energia);
        
        energia -= 1;
        
        if energia == 0 {println!("Sem energia");}
    }
    
    println!("============================================");
    
    // Exercício 3 - For
    
    let frutas = ["Maçã", "Banana", "Uva"];
    
    for fruta in frutas {
        println!("{}", fruta);
    }

    println!("============================================");
    
    // Desafio Extra ⭐
    
    for numero in (1..11).rev() {
        println!("{}", numero);

        if numero == 1 {println!("Foguete Lançado!");}
    }
    
}