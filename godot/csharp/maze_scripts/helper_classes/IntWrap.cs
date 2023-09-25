using System.Runtime.InteropServices.ComTypes;

namespace MazePerformanceGrade.csharp.maze_scripts.helper_classes;

public struct IntWrap
{
    
}

public static class GenericMathExtensions
{
    public static T AddGeneric<T>(T x, T y) where T : INumber<T>
    {
        return x + y;
    }
}