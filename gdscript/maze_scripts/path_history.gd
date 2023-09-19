extends Node3D

class_name PathHistory

@export var tracked : Node3D
@export var maze_config : MazeConfig


var path_history: Array[HistoricPosition]
var alt_history: Array[Vector2i]

func _get_current_tile_position() -> Vector2i:
	var local_position = to_local(tracked.global_position)
	return Vector2i(round(local_position.x), round(local_position.z))
	

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	var current_position := _get_current_tile_position()
	if(path_history.is_empty()):
		_log_next_position(current_position)
		return
	if(path_history.back().tile == current_position):
		# print(str(path_history.back().tile) + " == " + str(current_position))
		return
	_log_next_position(current_position)

func _log_next_position(current: Vector2i):
	if !maze_config.in_bounds(current):
		return
	path_history.append(HistoricPosition.new(current))
	alt_history.append(current)
	print("Logged player position: " + str(current))
	
