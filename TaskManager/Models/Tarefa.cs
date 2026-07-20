namespace TaskManager.Models;

public class Tarefa
{
    public string Titulo { get; set; }
    public string Descricao { get; set; }
    public DateTime DataCriacao { get; set; }

    public void ReceberTitulo()
    {
        Console.WriteLine("Digite o título da tarefa:");
        Titulo = Console.ReadLine();
    }

    public void ReceberDescricao()
    {
        Console.WriteLine("Digite a descrição da tarefa:");
        Descricao = Console.ReadLine();
    }

    public void ExibirTarefa()
    {
        DateTime dataAtual = DateTime.Today;

        Console.WriteLine("Título: " + Titulo);
        Console.WriteLine("Descrição: " + Descricao);
        Console.WriteLine("Data de criação: " + dataAtual.ToString("dd/MM/yyyy"));
    }
}