namespace aoc;

public static class Day_01
{
    public static long Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        Span<int> turns = stackalloc int[input.Length];
        for (var i = 0; i < input.Length; i++)
        {
            ReadOnlySpan<char> line = input[i].AsSpan();
            var val = AocHelpers.FastParseInt(line.Slice(1));
            turns[i] = input[i][0] == 'R' ? val  : - val;
        }
        
        timer.StartExecuting();
        var pos = 50;
        var zeroCount = 0;
        foreach (var t in turns)
        {
            // remaining turns
            pos += t % 100;

            // does the turn wrap around?
            if (pos > 99) pos -= 100;
            else if (pos < 0) pos += 100;
            
            if(pos == 0) zeroCount++;
        }

        timer.Stop();
        
        return zeroCount;
    }
    

    public static long Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        Span<int> turns = stackalloc int[input.Length];
        for (var i = 0; i < input.Length; i++)
        {
            ReadOnlySpan<char> line = input[i].AsSpan();
            var val = AocHelpers.FastParseInt(line.Slice(1));
            turns[i] = input[i][0] == 'R' ? val  : - val;
        }
        
        timer.StartExecuting();
        var pos = 50;
        var zeroCount = 0;

        foreach (var t in turns)
        {
            var oldPos = pos;
            
            // full turns
            zeroCount += Math.Abs(t / 100);

            // remaining turns
            var semi = t % 100;
            pos = oldPos + semi;

            // does the turn wrap around?
            var wrapOver = pos > 99;
            var wrapUnder  = pos < 0;

            if (wrapOver) pos -= 100;
            else if (wrapUnder) pos += 100;

            if (oldPos == 0) continue;
            
            if (wrapOver || wrapUnder ||  pos == 0)
                zeroCount++;
        }

        timer.Stop();

        return zeroCount;
    }
}