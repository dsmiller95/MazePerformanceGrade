using Godot;

namespace MazePerformanceGrade.csharp.helper_classes;

public static class RngExtensions
{
    /// <summary>
    /// generate a random vector between 0,0 and max, exclusive
    /// </summary>
    /// <param name="size">the exclusive max value</param>
    /// <returns></returns>
    public static Vector2I RandomVector(this RandomNumberGenerator rng, Vector2I size)
    {
        return new Vector2I(rng.RandiRange(0, size.X - 1), rng.RandiRange(0, size.Y - 1));
    }

    public static void ShuffleNaive<T>(this RandomNumberGenerator rng, T[] array)
    {
        for (int i = 0; i < array.Length; i++)
        {
            var swap = rng.RandiRange(i, array.Length - 1);
            (array[swap], array[i]) = (array[i], array[swap]);
        }
    }
}