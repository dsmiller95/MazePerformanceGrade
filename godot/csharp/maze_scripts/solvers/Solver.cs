using Godot;
using MazePerformanceGrade.csharp.maze_scripts.helper_classes;

namespace MazePerformanceGrade.csharp.maze_scripts.solvers;

public partial class Solver : Node
{
    public virtual HistoricPosition[] SolveMaze(Reachability mazeConfig, Vector2I position, int direction, Vector2I target)
    {
        var result = new HistoricPosition[0];
        return result;
    }
}