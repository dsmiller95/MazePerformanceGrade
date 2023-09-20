using Godot;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using MazePerformanceGrade.csharp.helper_classes;

public partial class MazeReplay : Node
{
	[Export] private FloorCreator floors;
	[Export] private MazeConfig mazeConfig;
	[Export] private Material highlightMaterial;
	[Export] private Material traveledMaterial;
	

	private bool isReplayingPath = false;
	public async Task BeginPathReplay(IList<HistoricPosition> path, float replayTimeMs)
	{
		GD.Print("trying to play path");
		if (isReplayingPath)
		{
			GD.PrintErr("currently playing path, cannot play another");
			return;
		}

		isReplayingPath = true;

		var totalTime = path.Last().GetTime() - path.First().GetTime();
		var adjustment = replayTimeMs / (float)totalTime;

		var sceneTree = GetTree();
		
		var lastPath = path[0];
		HighlightTile(lastPath.GetTile(), highlightMaterial);
		
		
		foreach (var location in path)
		{
			var adjustedMs = (location.GetTime() - lastPath.GetTime()) * adjustment;
			var timer = sceneTree.CreateTimer(adjustedMs / 1000);
			await ToSignal(timer, "timeout");
			HighlightTile(lastPath.GetTile(), traveledMaterial);
			HighlightTile(location.GetTile(), highlightMaterial);
			lastPath = location;
		}

		isReplayingPath = false;
	}
	
	private void HighlightTile(Vector2I tile, Material material)
	{
		GD.Print("replaying over tile " + tile);
		var floor = floors.floors_indexed[tile.X + tile.Y * mazeConfig.size.X];
		var mesh = floor.FindChild("MeshInstance3D") as MeshInstance3D;
		if (mesh == null)
		{
			GD.PrintErr("could not find mesh for floor");
			return;
		}

		mesh.MaterialOverlay = material;
	}
}
