extends Node3D

@export var tracked : Node3D

class HistoricPosition:
	var tile: Vector2i
	var time_ms: int
	
	func _init(tile: Vector2i):
		self.tile = tile
		time_ms = Time.get_ticks_msec()

var path_history: Array[HistoricPosition]
var other_history: Array[Vector2i]


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _get_current_tile_position() -> Vector2i:
	var local_position = to_local(tracked.global_position)
	return Vector2i(round(local_position.x), round(local_position.y))
	

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	var current_position := _get_current_tile_position()
	if(path_history.is_empty()):
		path_history.append(HistoricPosition.new(current_position))
		other_history.append(current_position)
		return
	if(path_history.back().tile == current_position):
		return
	path_history.append(HistoricPosition.new(current_position))
	other_history.append(current_position)
