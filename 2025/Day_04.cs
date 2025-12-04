namespace aoc;

public static class Day_04
{
    public static long Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var rows = input.Length;
        var columns = input[0].Length;
        
        var grid = new char[rows][];
        for (int r = 0; r < rows; r++)
        {
            grid[r] = input[r].ToCharArray();
        }
      
        timer.StartExecuting();

        var ans = 0;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < columns; c++)
            {
                var cell = grid[r][c];
                if (cell == '.')
                {
                    continue;
                }

                var surroundingRollsCount = 0;
                for (int i = -1; i <= 1; i++)
                {
                    for (int j = -1; j <= 1; j++)
                    {
                        var nr = r + i;
                        var nc = c + j;
                        if ((i == 0 && j == 0) || nr < 0 || nr >= rows || nc < 0 || nc >= columns)
                        {
                            // outside of grid
                            continue;
                        }

                        if (grid[nr][nc] != '@') 
                            continue;
                        
                        surroundingRollsCount++;
                        if (surroundingRollsCount > 3)
                        {
                            break;
                        }
                    }
                }
                
                if (surroundingRollsCount <= 3)
                {
                    ans++;
                }
            }
        }
        
        timer.Stop();
        return ans;
    }
    

    public static long Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var rows = input.Length;
        var columns = input[0].Length;
        var dirtySquares = new Queue<(int x, int y)>();

        var grid = new char[rows][];
        for (int r = 0; r < rows; r++)
        {
            grid[r] = input[r].ToCharArray();
            for (int c = 0; c < columns; c++)
            {
                if (grid[r][c] == '@')
                {
                    dirtySquares.Enqueue((r, c));
                }
            }
        }

        timer.StartExecuting();

        var ans = 0;

        while (dirtySquares.TryDequeue(out var pos))
        {
            var r = pos.x;
            var c = pos.y;
            
            var cell = grid[r][c];
            if (cell == '.')
            {
                continue;
            }

            var surroundingRollsCount = 0;
            for (int i = -1; i <= 1; i++)
            {
                for (int j = -1; j <= 1; j++)
                {
                    var nr = r + i;
                    var nc = c + j;
                    if ((i == 0 && j == 0) || nr < 0 || nr >= rows || nc < 0 || nc >= columns)
                    {
                        // outside of grid
                        continue;
                    }

                    if (grid[nr][nc] != '@') 
                        continue;
                    surroundingRollsCount++;
                    if (surroundingRollsCount > 3)
                    {
                        break;
                    }
                }
            }

            if (surroundingRollsCount > 3) 
                continue;
            
            ans++;
            
            grid[r][c] = '.';
            for (int i = -1; i <= 1; i++)
            {
                for (int j = -1; j <= 1; j++)
                {
                    var nr = r + i;
                    var nc = c + j;
                    if ((i == 0 && j == 0) || nr < 0 || nr >= rows || nc < 0 || nc >= columns)
                    {
                        // outside of grid
                        continue;
                    }
                        
                    dirtySquares.Enqueue((nr, nc));
                }
            }
        }
        
        timer.Stop();
        return ans;
    }

}