using Godot;

namespace MazePerformanceGrade.csharp.helper_classes;

public readonly record struct TileEdge
{
    public Vector2I A { get; }
    public Vector2I B { get; }
    
    public TileEdge(Vector2I a, Vector2I b)
    {
        var delta = b - a;
        if(delta.X != 0 && delta.Y != 0)
            throw new System.ArgumentException("edges must be either horizontal or vertical");
        // enforce a consistent ordering. order by x coord first, then ycoord if x is equal
        //  this is to make sure that the same edge is always represented the same way
        if (delta.X != 0)
        {
            if (delta.X < 0) (b, a) = (a, b);
        }
        else
        {
            if(delta.Y < 0) (b, a) = (a, b);
        }
        A = a;
        B = b;
    }

    
    public static Vector3 To3DFrom2D(Vector2I flat)
    {
        return new Vector3(flat.X, 0, flat.Y);
    }

    public int GetStableIdInDomain(Vector2I size)
    {
        var minVec = Utils.PartsWise(A, B, Mathf.Min);
        var maxVec = Utils.PartsWise(A, B, Mathf.Max);
        
        var min = minVec.X + minVec.Y * size.X;
        var max = maxVec.X + maxVec.Y * size.X;
        return min + max * (size.X * size.Y);
    }
    
    public void SpawnWallAtEdge(PackedScene wallPrefab, Node parent)
    {
        var worldA = To3DFrom2D(A);
        var worldB = To3DFrom2D(B);
        
        var center = (worldA + worldB) / 2;
        var forward = (worldB - worldA).Normalized();
        var newWall = wallPrefab.InstantiateForceCast<Node3D>();
        newWall.Transform = new Transform3D(Basis.LookingAt(forward, Vector3.Up), center);
        parent.AddChild(newWall);
    }
}