using System;
using System.Runtime.CompilerServices;
using Godot;

namespace MazePerformanceGrade.csharp.helper_classes;

public static class Utils
{
    public static T InstantiateForceCast<T>(
        this PackedScene scene,
        [CallerArgumentExpression("scene")] string? paramName = null)
        where T : Node
    {
        var newNode = scene.Instantiate();
        if (newNode is not T castedNode)
        {
            throw new ArgumentException("floorPrefab must be a Node3D", paramName);
        }
        return castedNode;
    }
    
    public static Vector2I PartsWise(Vector2I a, Vector2I b, Func<int, int, int> func)
    {
        return new Vector2I(func(a.X, b.X), func(a.Y, b.Y));
    }
}