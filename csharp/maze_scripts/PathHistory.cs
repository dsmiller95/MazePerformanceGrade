using Godot;
using System;
using System.Collections.Generic;
using System.Linq;
using MazePerformanceGrade.csharp.helper_classes;

public partial class PathHistory : Node3D
{
	[Export] private Node3D? tracked;
	[Export] private MazeConfig? mazeConfig;

	public List<HistoricPosition> pathHistory { get; private set; } = new List<HistoricPosition>();
	
	public override void _PhysicsProcess(double delta)
	{
		var current = GetCurrentTilePosition();
		if (pathHistory.Count <= 0)
		{
			LogNextPosition(current);
			return;
		}

		var lastTile = pathHistory.Last().GetTile();
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
