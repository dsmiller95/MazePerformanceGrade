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

    public static GodotObject[] To(HistoricPosition[] positions)
    {
        var result = new GodotObject[positions.Length];
        for (int i = 0; i < positions.Length; i++)
        {
            result[i] = positions[i].obj;
        }
        return result;
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
        var variant = obj.Get("time");
        if(variant.VariantType != Variant.Type.Int)
            throw new System.InvalidOperationException("Expected Int");
        return variant.AsInt32();
    }
}