using System;
using System.Collections.Generic;
using System.Linq;
using Godot;
using MazePerformanceGrade.csharp.maze_scripts.helper_classes;

namespace MazePerformanceGrade.csharp.maze_scripts;

public partial class PathHistory : Node3D
{
	[Export] private Node3D? tracked;
	[Export] private MazeConfig? mazeConfig;

	public List<HistoricPosition> pathHistory { get; } = new();
	
	public override void _PhysicsProcess(double delta)
	{
		var current = GetCurrentTilePosition();
		if (pathHistory.Count <= 0)
		{
			LogNextPosition(current);
			return;
		}

		var lastTile = pathHistory.Last().Tile;
		if (lastTile == current)
		{
			return;
		}
		
		LogNextPosition(current);
	}
	
	private Vector2I GetCurrentTilePosition()
	{
		ArgumentNullException.ThrowIfNull(tracked);
		var localPosition = ToLocal(tracked.GlobalPosition);
		return new Vector2I(Mathf.RoundToInt(localPosition.X), Mathf.RoundToInt(localPosition.Z));
	}

	private void LogNextPosition(Vector2I current)
	{
		ArgumentNullException.ThrowIfNull(mazeConfig);
		if (!mazeConfig.in_bounds(current))
		{
			return;
		}

		pathHistory.Add(new HistoricPosition(current));
		GD.Print("Logged player position: " + current);
	}
}