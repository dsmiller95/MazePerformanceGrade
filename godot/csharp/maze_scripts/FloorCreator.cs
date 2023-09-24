using System;
using Godot;
using MazePerformanceGrade.csharp.maze_scripts.helper_classes;

namespace MazePerformanceGrade.csharp.maze_scripts;

public partial class FloorCreator : Node
{
	// #WEIRD we still can't enforce the type of a packed scene reference in C#
	[Export] private PackedScene? floorPrefab;
	[Export] private MazeConfig? mazeConfig;

	public Node3D[] floors_indexed { get; private set; } = Array.Empty<Node3D>();

	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		ArgumentNullException.ThrowIfNull(mazeConfig);
		ArgumentNullException.ThrowIfNull(floorPrefab);
		var size = mazeConfig.size;
		floors_indexed = new Node3D[size.X * size.Y];
		for (int x = 0; x < size.X; x++)
		{
			for (int y = 0; y < size.Y; y++)
			{
				var newNode = floorPrefab.InstantiateForceCast<Node3D>();
				newNode.Position = new Vector3(x, 0, y);
				AddChild(newNode);
				floors_indexed[x + y * size.X] = newNode;
			}
		}
	}
}