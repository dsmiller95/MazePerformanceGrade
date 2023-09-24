using Godot;

namespace MazePerformanceGrade.csharp.maze_scripts.helper_classes;

public readonly record struct HistoricPosition(Vector2I Tile, ulong TimeMs)
{
    public readonly Vector2I Tile = Tile;
    public readonly ulong TimeMs = TimeMs;

    public HistoricPosition(Vector2I tile) : this(tile, Time.GetTicksMsec())
    {
    }
}