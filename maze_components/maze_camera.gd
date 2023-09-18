@tool
extends Camera3D

@export var maze_config: MazeConfig

# Called when the node enters the scene tree for the first time.
func _ready():
	position_camera()
	pass # Replace with function body.
# Called when the node enters the scene tree for the first time.
func _run():
	position_camera()
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	position_camera()
	pass

func position_camera():
	size = max(maze_config.size.x, maze_config.size.y) * 2
	position = MazeConfig.to_3d_from_2d(maze_config.size) / 2 + Vector3.UP * 2
