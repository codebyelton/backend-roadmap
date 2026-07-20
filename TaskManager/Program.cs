using TaskManager.Models;

class Program
{
    static void Main(string[] args)
    {
        Tarefa tarefa = new Tarefa();

        tarefa.ReceberTitulo();
        tarefa.ReceberDescricao();
        tarefa.ExibirTarefa();
        
    }
}