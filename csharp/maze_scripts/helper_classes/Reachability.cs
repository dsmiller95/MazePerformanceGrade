using System.Collections.Generic;
using System.Linq;
using Godot;

namespace MazePerformanceGrade.csharp.helper_classes;

public partial class Reachability : GodotObject
{
    public static readonly Vector2I[] Neighbors = {
        Vector2I.Up,
        Vector2I.Right,
        Vector2I.Down,
        Vector2I.Left,
    };
    
    private readonly bool[,] reachable;
    public Vector2I size { get; init; }
    private readonly List<TileEdge> navigableEdges = new();
    
    public Reachability(Vector2I size)
    {
        reachable = new bool[size.X, size.Y];
        this.size = size;
    }

    public void AssertFullyReachable()
    {
        for (int x = 0; x < size.X; x++)
        {
            for (int y = 0; y < size.Y; y++)
            {
                var pos = new Vector2I(x, y);
                System.Diagnostics.Debug.Assert(Reachable(pos), $"Cell {x},{y} is not reachable");
            }
        }
    }
    
    public bool InBounds(Vector2I position)
    {
        if (position.X < 0 || position.Y < 0)
            return false;
        if (position.X >= size.X || position.Y >= size.Y)
            return false;
        return true;
    }

    public bool Reachable(Vector2I nextSearch)
    {
        return reachable[nextSearch.X, nextSearch.Y];
    }

    public bool Traversable(TileEdge edge)
    {
        return navigableEdges.Contains(edge);
    }

    public void ReachBetween(Vector2I alreadyReached, Vector2I newlyReached)
    {
        if (!Reachable(alreadyReached))
        {
            GD.PrintErr("trying to reach from unreached cell");
        }
        if (Reachable(newlyReached))
        {
            GD.PrintErr("reaching into an already reached cell");
        }
        reachable[newlyReached.X, newlyReached.Y] = true;
        navigableEdges.Add(new TileEdge(alreadyReached, newlyReached));
    }

    public TileEdge[] GetWalls()
    {
        var allPossibleWalls = AllEdges(size);
        var walls =  allPossibleWalls.Except(navigableEdges).ToArray();

        return walls;
    }

    public static TileEdge[] AllEdges(Vector2I size)
    {
        var edges = new List<TileEdge>();
        for (int x = 0; x < size.X - 1; x++)
        {
            for (int y = 0; y < size.Y; y++)
            {
                edges.Add(new TileEdge(
                    new Vector2I(x, y),
                    new Vector2I(x + 1, y)
                    ));
            }
        }
        for (int x = 0; x < size.X; x++)
        {
            for (int y = 0; y < size.Y - 1; y++)
            {
                edges.Add(new TileEdge(
                    new Vector2I(x, y),
                    new Vector2I(x, y + 1)
                ));
            }
        }
        return edges.ToArray();
    }
}