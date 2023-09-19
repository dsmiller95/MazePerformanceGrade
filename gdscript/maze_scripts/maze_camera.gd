@tool
extends Camera3D

@export var maze_config: MazeConfig

# Called when the node enters the scene tree for the first time.
func _ready():
	position_camera()

func position_camera():
	size = max(maze_config.size.x, maze_config.size.y) * 2
	position = MazeConfig.to_3d_from_2d(maze_config.size) / 2 + Vector3.UP * 2
