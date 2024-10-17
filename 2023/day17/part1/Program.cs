var lines = File.ReadAllLines("../testInput.txt");

var graph = new int[lines.Length][];
for(int i = 0; i < lines.Length; i++)
{
    graph[i] = lines[i].Select(c => int.Parse(c.ToString())).ToArray();
}

const int Infinity = int.MaxValue;

int source = 0; // Choose the source vertex

FindShortestPath(graph, source);

static void FindShortestPath(int[][] graph, int source)
{
    int vertices = graph.GetLength(0);
    int[] distances = new int[vertices];
    bool[] visited = new bool[vertices];

    // Initialize distances array with Infinity and mark all vertices as unvisited
    for (int i = 0; i < vertices; i++)
    {
        distances[i] = Infinity;
        visited[i] = false;
    }

    distances[source] = 0;

    // Find shortest path for all vertices
    for (int count = 0; count < vertices - 1; count++)
    {
        int u = MinimumDistance(distances, visited);
        visited[u] = true;

        for (int v = 0; v < vertices; v++)
        {
            if (!visited[v] && graph[u][v] != 0 && distances[u] != Infinity &&
                distances[u] + graph[u][v] < distances[v])
            {
                distances[v] = distances[u] + graph[u][v];
            }
        }
    }

    // Print the shortest distances from the source vertex
    Console.WriteLine("Shortest distances from source vertex " + source + " to all other vertices:");
    for (int i = 0; i < vertices; i++)
    {
        Console.WriteLine($"Vertex {i}: Distance = {(distances[i] == Infinity ? "Infinity" : distances[i].ToString())}");
    }
}

static int MinimumDistance(int[] distances, bool[] visited)
{
    int min = Infinity;
    int minIndex = -1;

    for (int v = 0; v < distances.Length; v++)
    {
        if (!visited[v] && distances[v] <= min)
        {
            min = distances[v];
            minIndex = v;
        }
    }

    return minIndex;
}