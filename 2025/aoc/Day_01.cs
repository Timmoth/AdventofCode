namespace aoc;

public static class Day_01
{
    public static int Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var turns = new int[input.Length];
        for (var j = 0; j < turns.Length; j++)
        {
            var line = input[j];
            turns[j] = line[0] == 'R' ? int.Parse(line.AsSpan()[1..]) : -int.Parse(line.AsSpan()[1..]);
        }
        
        timer.StartExecuting();
        var pos = 50;
        var zeroCount = 0;
        foreach (var t in turns)
        {
            var oldPos = pos;

            // remaining turns
            var semi = t % 100;
            pos = oldPos + semi;

            // does the turn wrap around?
            var wrapOver = pos > 99;
            var wrapUnder  = pos < 0;

            if (wrapOver) pos -= 100;
            else if (wrapUnder) pos += 100;

            if(pos == 0) zeroCount++;
        }

        timer.Stop();
        
        return zeroCount;
    }
    

    public static int Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var turns = new int[input.Length];
        for (var j = 0; j < turns.Length; j++)
        {
            var line = input[j];
            turns[j] = line[0] == 'R' ? int.Parse(line.AsSpan()[1..]) : -int.Parse(line.AsSpan()[1..]);
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