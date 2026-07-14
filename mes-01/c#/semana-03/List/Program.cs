//Exercício 1
List<string> nomes = new List<string>();

nomes.Add("Alice");
nomes.Add("Bob");
nomes.Add("Charlie");

foreach (string nome in nomes)
{
    Console.WriteLine(nome);
}

Console.WriteLine("================================");

//Exercício 2
List<int> numeros = new List<int>();
numeros.Add(10);
numeros.Add(20);
numeros.Add(30);
numeros.Add(40);

numeros.Remove(20); // Remove o número 20 da lista

foreach (int numero in numeros)
{
    Console.WriteLine(numero);
}

Console.WriteLine("================================");

//Exercício 3
List<string> cidades = new List<string>();

cidades.Add("São Paulo");
cidades.Add("Rio de Janeiro");
cidades.Add("Belo Horizonte");
cidades.Add("Curitiba");

foreach (string cidade in cidades)
{
    Console.WriteLine(cidade);
}

Console.WriteLine(cidades.Count); // Exibe a quantidade de elementos na lista