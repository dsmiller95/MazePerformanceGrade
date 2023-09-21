using Godot;
using System;

public partial class MazeCamera : Camera3D
{
	[Export] MazeConfig mazeConfig;
	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		PositionCamera();
	}

	private void PositionCamera()
	{
		Size = Mathf.Max(mazeConfig.size.X, mazeConfig.size.Y) * 2;
		Position = MazeConfig.to_3d_from_2d(mazeConfig.size) / 2 + Vector3.Up * 2;
	}
}
