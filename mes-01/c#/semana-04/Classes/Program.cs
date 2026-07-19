using System;
class Program
{
    static void Main()
    {
        // Exercício 1 e 2
        Cachorro cachorro = new Cachorro();

        cachorro.nome = "Scooby";
        cachorro.idade = 7;

        cachorro.mostrar_info();
        cachorro.Latir();

        //Desafio Extra
        Personagem personagem = new Personagem();

        personagem.nome = "Guerreiro";
        personagem.vida = 100;
        personagem.nivel = 1;

        personagem.MostrarStatus();
    }

    // Exercício 1 e 2
    class Cachorro
    {
        public string nome;
        public int idade;

        public void mostrar_info()
        {
            Console.WriteLine("Nome: " + nome);
            Console.WriteLine("Idade: " + idade);
        }

        public void Latir()
        {
            Console.WriteLine("Au Au!");
        }
    }

    //Desafio Extra
    class Personagem
    {
        public string nome;
        public int vida;
        public int nivel;

        public void MostrarStatus()
        {
            Console.WriteLine("Nome: " + nome);
            Console.WriteLine("Vida: " + vida);
            Console.WriteLine("Nível: " + nivel);
        }
    }
}