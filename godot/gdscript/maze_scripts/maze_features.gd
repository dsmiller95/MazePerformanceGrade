extends Node3D

@export var maze_data : MazeConfig
@export var exit_feature : Node3D
@export var entry_feature : Node3D

# Called when the node enters the scene tree for the first time.
func _ready():
	if exit_feature:
		exit_feature.global_position = to_global(MazeConfig.to_3d_from_2d(maze_data.exit))
	if entry_feature:
		entry_feature.global_position = to_global(MazeConfig.to_3d_from_2d(maze_data.entry))
