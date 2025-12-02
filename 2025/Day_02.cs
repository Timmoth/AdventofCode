namespace aoc;

public static class Day_02
{
    public static long Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var line = input[0].AsSpan();
        var numbers = new List<long>();
        var numStart = 0;

        for (var i = 0; i < line.Length; i++)
        {
            if (line[i] is not ',' and not '-') 
                continue;
            numbers.Add(AocHelpers.FastParseLong(line.Slice(numStart, i - numStart)));
            numStart = i + 1;
        }
        
        numbers.Add(AocHelpers.FastParseLong(line.Slice(numStart)));

        timer.StartExecuting();
        Span<char> buf = stackalloc char[64];

        long ans = 0;
        for (int i = 0; i < numbers.Count; i += 2)
        {
            var start = numbers[i];
            var end = numbers[i + 1];

            // iterate over each number in the range
            for (long j = start; j <= end; j++)
            {
                j.TryFormat(buf, out int len);
                var st = buf.Slice(0, len);
                if (st.Length % 2 == 1)
                {
                    // Skip odd
                    continue;
                }
                
                var pass = true;
                var half = st.Length / 2;
                for (var k = 0; k < half; k++)
                {
                    // check each char in the first half has a corresponding char in the second half
                    if (st[k] == st[half + k]) continue;
                    
                    // exit early
                    pass = false;
                    break;
                }

                if (pass)
                {
                    // all characters are repeated
                    ans += j;
                }
            }
        }
        
        timer.Stop();
        return ans;
    }
    

    public static long Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var line = input[0].AsSpan();
        var numbers = new List<long>();
        var numStart = 0;

        for (var i = 0; i < line.Length; i++)
        {
            if (line[i] is not ',' and not '-') 
                continue;
            numbers.Add(AocHelpers.FastParseLong(line.Slice(numStart, i - numStart)));
            numStart = i + 1;
        }
        
        numbers.Add(AocHelpers.FastParseLong(line.Slice(numStart)));

        timer.StartExecuting();
        Span<char> buf = stackalloc char[64];

        long ans = 0;
        for (int i = 0; i < numbers.Count; i += 2)
        {
            var start = numbers[i];
            var end = numbers[i + 1];

            for (long j = start; j <= end; j++)
            {
                j.TryFormat(buf, out int len);
                var st = buf.Slice(0, len);
                
                // check all sequence lengths
                for (int k = 1; k <= st.Length / 2; k++)
                {
                    if (!IsRepeatedN(st, k)) 
                        continue;
                    
                    // found a repeated sequence, exit
                    ans += j;
                    break;
                }
            }
        }
        
        timer.Stop();
        return ans;
    }
    
    static bool IsRepeatedN(ReadOnlySpan<char> value, int n)
    {
        var len = value.Length;

        if (len % n != 0)
            return false;

        var seq = value.Slice(0, n);

        for (var i = n; i < len; i += n)
        {
            var next = value.Slice(i, n);
            if (!seq.SequenceEqual(next))
                return false;
        }

        return true;
    }


}