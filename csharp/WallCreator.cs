using System;
using Godot;
using MazePerformanceGrade.csharp.helper_classes;

public partial class WallCreator : Node
{
	[Export] private PackedScene wallPrefab;
	[Export] private MazeConfig mazeConfig;
	
	private TileEdge[] walls = Array.Empty<TileEdge>();
	private Reachability reachable = null!;
	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		ArgumentNullException.ThrowIfNull(wallPrefab);
		ArgumentNullException.ThrowIfNull(mazeConfig);
		
		reachable = RandomizedDfsWalls(mazeConfig.size, new RandomNumberGenerator());
		walls = reachable.GetWalls();
		foreach (var wall in walls)
		{
			wall.SpawnWallAtEdge(wallPrefab, this);
		}
	}

	private Reachability RandomizedDfsWalls(Vector2I size, RandomNumberGenerator rng)
	{
		var reach = new Reachability(size);
		var origin = rng.RandomVector(size);
		
		InternalRandomizeDfsWalls(reach, origin, rng);
		reach.AssertFullyReachable();

		return reach;
	}

	private void InternalRandomizeDfsWalls(Reachability reach, Vector2I cell, RandomNumberGenerator rng)
	{
		var indexes = new int[] { 0, 1, 2, 3 };
		rng.ShuffleNaive(indexes);
		foreach (var index in indexes)
		{
			var nextSearch = cell + Reachability.Neighbors[index];
			if(!reach.InBounds(nextSearch) || reach.Reachable(nextSearch)) continue;
			reach.ReachBetween(cell, nextSearch);
			InternalRandomizeDfsWalls(reach, nextSearch, rng);
		}
	}
}
