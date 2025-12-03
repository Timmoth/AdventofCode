namespace aoc;

public static class Day_03
{
    public static long Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var cellCount = input[0].Length;
        var batteryCount = input.Length;
        Span<byte> batteries = stackalloc byte[batteryCount * cellCount];

        for (int i = 0; i < batteryCount; i++)
        {
            var battery = input[i].AsSpan();
            for (int j = 0; j < cellCount; j++)
            {
                batteries[i * cellCount + j] = (byte)(battery[j] - '0');
            }
        }
      
        timer.StartExecuting();
        var ans = 0;

        for (int i = 0; i < batteryCount; i++)
        {
            var a = 0;
            var b = 0;
            for (int j = 0; j < cellCount - 1; j++)
            {
                var cell = batteries[i * cellCount + j];
                if (cell > a)
                {
                    a = cell;
                    b = 0;
                }else if (cell > b)
                {
                    b = cell;
                }
            }

            ans += a * 10 + Math.Max(b, batteries[i * cellCount + cellCount - 1]);
        }
        
        timer.Stop();
        return ans;
    }
    

    public static long Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var cellCount = input[0].Length;
        var batteryCount = input.Length;
        Span<byte> batteries = stackalloc byte[batteryCount * cellCount];

        for (int i = 0; i < batteryCount; i++)
        {
            var battery = input[i].AsSpan();
            for (int j = 0; j < cellCount; j++)
            {
                batteries[i * cellCount + j] = (byte)(battery[j] - '0');
            }
        }
      
        timer.StartExecuting();
        long ans = 0;

        for (int i = 0; i < batteryCount; i++)
        {
            ans += GetNextLargestNumber(batteries.Slice(i * cellCount, cellCount), 0, 12);
        }
        
        timer.Stop();
        return ans;
    }

    private static long GetNextLargestNumber(Span<byte> cells, int index, int size)
    {
        if (size <= 0)
        {
            return -1;
        }

        long num = 0;

        for (int i = index; i < cells.Length - size + 1; i++)
        {
            var newIndex = i;
            long a = 0;
            for (int j = index; j < cells.Length - size + 1; j++)
            {
                var cell = cells[j];
                if (cell > a)
                {
                    a = cell;
                    newIndex = j;
                }
            }
            if (size -1 == 0)
            {
                return a * (long) Math.Pow(10, size - 1) + num;
            }
            
            num = GetNextLargestNumber(cells, newIndex + 1, size - 1);
            if (num >= 0)
            {
                return a * (long) Math.Pow(10, size - 1) + num;
            }
        }
        
        return -1;
    }

}