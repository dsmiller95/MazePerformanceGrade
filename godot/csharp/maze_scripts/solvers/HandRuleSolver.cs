using System.Collections.Generic;
using Godot;
using MazePerformanceGrade.csharp.maze_scripts.helper_classes;

namespace MazePerformanceGrade.csharp.maze_scripts.solvers;

public partial class HandRuleSolver : Solver
{
    [Export] private bool keepRight = true;
    
    public override HistoricPosition[] SolveMaze(Reachability maze, Vector2I position, int direction, Vector2I target)
    {
        var path = new List<HistoricPosition>();
        uint delayMs = 500;
        uint currentStep = 0;
        var maxSteps = Mathf.Pow(maze.size.X * maze.size.Y, 2) ;
        
        var checkDir = keepRight ? 1 : 3;
        var searchDir = keepRight ? 3 : 1;
        
        path.Add(new HistoricPosition(position, currentStep * delayMs));

        while (position != target && currentStep < maxSteps)
        {
            var forwardTile = position + Reachability.Neighbors[direction];
            var forwardEdge = new TileEdge(position, forwardTile);
            var canMoveForward = maze.InBounds(forwardTile) && maze.Traversable(forwardEdge);

            if (canMoveForward)
            {
                position = forwardTile;
                direction = (direction + checkDir) % 4;
                path.Add(new HistoricPosition(position, currentStep * delayMs));
            }
            else
            {
                direction = (direction + searchDir) % 4;
            }
            
            currentStep++;
        }

        return path.ToArray();
    }
}