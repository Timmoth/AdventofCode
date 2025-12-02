using System.Runtime.CompilerServices;

namespace aoc;

public static class AocHelpers
{
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static int FastParseInt(ReadOnlySpan<char> span)
    {
        var value = 0;
        foreach (var t in span)
            value = value * 10 + (t - '0');

        return value;
    }
    
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static long FastParseLong(ReadOnlySpan<char> span)
    {
        long value = 0;
        foreach (var t in span)
            value = value * 10 + (t - '0');

        return value;
    }
}