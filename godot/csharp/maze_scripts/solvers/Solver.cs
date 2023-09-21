using Godot;
using System;
using MazePerformanceGrade.csharp.helper_classes;

public partial class Solver : Node
{
    public virtual HistoricPosition[] SolveMaze(Reachability mazeConfig, Vector2I position, int direction, Vector2I target)
    {
        var result = new HistoricPosition[0];
        return result;
    }
}
