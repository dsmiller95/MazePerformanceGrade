using Godot;
using System;

public partial class MazeFeatures : Node3D
{
	[Export] private MazeConfig? mazeConfig;
	[Export] private Node3D? exitFeature;
	[Export] private Node3D? entryFeature;
	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		ArgumentNullException.ThrowIfNull(mazeConfig);
		if (exitFeature != null)
			exitFeature.GlobalPosition = ToGlobal(MazeConfig.to_3d_from_2d(mazeConfig.exit));
		if (entryFeature != null)
			entryFeature.GlobalPosition = ToGlobal(MazeConfig.to_3d_from_2d(mazeConfig.entry));
	}
}
