using System.Collections.Generic;
using Godot;

namespace MazePerformanceGrade.csharp.helper_classes;

public readonly struct HistoricPosition
{
    private readonly GodotObject obj;

    public HistoricPosition(Vector2I tile, int time)
    {
        var script = GD.Load<GDScript>("res://gdscript/maze_scripts/helper_classes/historic_position.gd");
        obj = (GodotObject)script.New(tile, time);
    }
    public HistoricPosition(Vector2I tile)
    {
        var script = GD.Load<GDScript>("res://gdscript/maze_scripts/helper_classes/historic_position.gd");
        obj = (GodotObject)script.New(tile);
    }
    
    public HistoricPosition(GodotObject godotObject)
    {
        obj = godotObject;
    }
    public static HistoricPosition[] From(GodotObject[] godotObjects)
    {
        var result = new HistoricPosition[godotObjects.Length];
        for (int i = 0; i < godotObjects.Length; i++)
        {
            result[i] = new HistoricPosition(godotObjects[i]);
        }
        return result;
    }

    public static Godot.Collections.Array To(IList<HistoricPosition> positions)
    {
        var result = new Variant[positions.Count];
        for (int i = 0; i < positions.Count; i++)
        {
            result[i] = positions[i].obj;
        }
        return new Godot.Collections.Array(result);
    }

    public Vector2I GetTile()
    {
        var variant = obj.Get("tile");
        if(variant.VariantType != Variant.Type.Vector2I)
            throw new System.InvalidOperationException("Expected Vector2I");
        return variant.AsVector2I();
    }
    public int GetTime()
    {
        var variant = obj.Get("time_ms");
        if(variant.VariantType != Variant.Type.Int)
            throw new System.InvalidOperationException("Expected Int");
        return variant.AsInt32();
    }
}