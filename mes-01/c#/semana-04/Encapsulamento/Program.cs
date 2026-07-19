using System;
class Program
{
    static void Main()
    {
        //Exercício 1
        Console.WriteLine("Exercício 1");
        Conta conta = new Conta();

        conta.Depositar(1000);
        conta.MostrarSaldo();

        Console.WriteLine("");

        //Exercício 2
        Console.WriteLine("Exercício 2");
        Personagem personagem = new Personagem();

        Console.WriteLine("Vida Antes: " + personagem.Vida);
        personagem.ReceberDano(20);
        Console.WriteLine("Vida Atual: " + personagem.Vida);

        Console.WriteLine("");

        //Desafio Extra
        Console.WriteLine("Desafio Extra");
        Cofrinho cofrinho = new Cofrinho();

        cofrinho.AdicionaDinheiro(100);
        cofrinho.RetirarDinheiro(100);
    }

    //Exercício 1
    class Conta
    {
        private double saldo;

        public void Depositar(double valor)
        {
            saldo += valor;
        }

        public void MostrarSaldo()
        {
            Console.WriteLine("Saldo: " + saldo);
        }
    }

    //Exercício 2
    class Personagem
    {

        public int Vida { get; private set; } = 100;

        public void ReceberDano(int dano)
        {
            Vida -= dano;
        }

    }

    //Desafio Extra
    class Cofrinho
    {
        private double dinheiro;

        public void AdicionaDinheiro(int addSaldo)
        {
            dinheiro += addSaldo;
        }

        public void RetirarDinheiro(int removeSaldo)
        {
            if (dinheiro >= removeSaldo)
            {
                dinheiro -= removeSaldo;
                MostrarSaldo();
            }
            else
            {
                Console.WriteLine("Você não têm esse Saldo.");
                Console.WriteLine("Saldo Atual: " + dinheiro);
            }

        }

        public void MostrarSaldo()
        {
            Console.WriteLine("Saldo Atual: " + dinheiro);
        }
    }
}