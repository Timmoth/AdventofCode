namespace aoc;

public static class Day_05
{
    public static long Part1(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var ranges = new List<(long start, long end)>();
        var nums = new List<long>();

        var firstPartOfInput = true;
        for(int i = 0; i < input.Length; i++)
        {
            var line = input[i].AsSpan();
            
            if (line.Length == 0)
            {
                firstPartOfInput = false;
                continue;
            }

            if (firstPartOfInput)
            {
                var parts = line.Split('-');

                parts.MoveNext();
                var a = AocHelpers.FastParseLong(line[parts.Current]); // parts.Current is ReadOnlySpan<char>

                parts.MoveNext();
                var b = AocHelpers.FastParseLong(line[parts.Current]);

                ranges.Add((a, b));

            }
            else
            {
                nums.Add(AocHelpers.FastParseLong(line));
            }
        }
      
        ranges.Sort(static (a, b) => a.start.CompareTo(b.start));

        timer.StartExecuting();
        var ans = 0;

        foreach (var num in nums)
        {
            foreach (var r in ranges)
            {
                if (r.start <= num && num <= r.end)
                {
                    ans += 1;
                    break;
                }

                if (num < r.start)
                {
                    break;
                }
            }
        }
  
        
        timer.Stop();
        return ans;
    }
    
    public static long Part2(SolutionTimer timer, string[] input)
    {
        timer.StartParsing();

        var ranges = new List<(long start, long end)>();

        for(int i = 0; i < input.Length; i++)
        {
            var line = input[i].AsSpan();
            
            if (line.Length == 0)
            {
                break;
            }

            var parts = line.Split('-');

            parts.MoveNext();
            var a = AocHelpers.FastParseLong(line[parts.Current]); // parts.Current is ReadOnlySpan<char>

            parts.MoveNext();
            var b = AocHelpers.FastParseLong(line[parts.Current]);

            ranges.Add((a, b));
        }
        ranges.Sort(static (a, b) => a.start.CompareTo(b.start));
        ranges = MergeRanges(ranges);
        timer.StartExecuting();
        long ans = 0;

        foreach (var r in ranges)
        {
            ans += (r.end - r.start + 1);
        }
        
        timer.Stop();
        return ans;
    }

    static List<(long start, long end)> MergeRanges(List<(long start, long end)> ranges)
    {
        var merged = new List<(long start, long end)>(ranges.Count);

        var cur = ranges[0];

        for (int i = 1; i < ranges.Count; i++)
        {
            var next = ranges[i];

            if (next.start <= cur.end)
            {
                if (next.end > cur.end)
                    cur.end = next.end;
            }
            else
            {
                merged.Add(cur);
                cur = next;
            }
        }

        merged.Add(cur);
        return merged;
    }

    
}