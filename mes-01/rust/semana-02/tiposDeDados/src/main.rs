fn main() {
    
    //Variável Inteira Chamada "vida"
    let vida: i32 = 100;

    //Variável Float chamada "altura"
    let altura: f64 = 1.75;

    //Variável Booleana Chamada "jogando"
    let jogando: bool = true;

    //Variável Char chamada "rank"
    let rank: char = 'A';

    //Tupla contendo nome, idade e nível
    let jogador: (&str, i32, &str) = ("João", 25, "Avançado");

    //Array com 5 números
    let numeros: [i32; 5] = [10, 20, 30, 40, 50];

    println!("Vida: {}", vida);
    println!("Altura: {}", altura);
    println!("Jogando: {}", jogando);
    println!("Rank: {}", rank);
    println!("Jogador: Nome: {}, Idade: {}, Nível: {}", jogador.0, jogador.1, jogador.2);
    println!("Números: {:?}", numeros);
}
