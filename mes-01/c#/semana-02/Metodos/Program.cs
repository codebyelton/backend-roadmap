using System;

class Program
{
    static void Main()
    {
        BoasVindas();
        
        MostrarNome("Elton");
        
        int resultado = Multiplicar(2,2);
        
        Console.WriteLine("Resultado: " + resultado);
    }
    
    //Ex 1
    static void BoasVindas(){
        Console.WriteLine("Bem-vindo ao curso de C#");
    }
    
    //Ex 2 
    static void MostrarNome(string nome){
        Console.WriteLine("Olá, " + nome + "!");
    }
    
    //Desafio Extra
    static int Multiplicar(int numero1, int numero2){
        return numero1 * numero2;
    }
}