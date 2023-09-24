using System;
using System.Runtime.CompilerServices;
using Godot;

namespace MazePerformanceGrade.csharp.maze_scripts.helper_classes;

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
            throw new ArgumentException("floorPrefab must be a " + typeof(T).Name, paramName);
        }
        return castedNode;
    }
}