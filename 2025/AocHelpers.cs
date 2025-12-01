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
}