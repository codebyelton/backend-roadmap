fn main() {
    boas_vindas();
    
    mostrar_idade(18);
    
    let resultado = dobro(8);
    
    println!("{}", resultado);
}

//Exercício 1 
fn boas_vindas(){
    println!("Bem-vindo ao Rust!");
}

//Exercício 2
fn mostrar_idade(idade: i32){
    println!("Você têm {} anos", idade);
}

//Exercício 3
fn dobro(numero: i32) -> i32 {
    numero * 2
}