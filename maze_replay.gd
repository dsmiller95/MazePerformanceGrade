extends Node3D
class_name MazeReplay

@export var floors: FloorGenerator
@export var maze_data: MazeConfig

@export var highlight_material: Material
@export var traveled_material: Material

var is_replaying_path := false

func begin_path_replay(path: Array[HistoricPosition], replay_time_ms: int = 10000):
	if(is_replaying_path):
		print("ERROR: currently playing path, cannot play another")
		return
	is_replaying_path = true
	
	var total_time = path.back().time_ms - path[0].time_ms
	var adjustment = replay_time_ms / float(total_time)
	
	var scene_tree := get_tree()
	
	var last_path = path[0]
	highlight_tile(last_path.tile, highlight_material)
	
	for location in path:
		var adjusted_ms = (location.time_ms - last_path.time_ms) * adjustment
		await scene_tree.create_timer(adjusted_ms / 1000.0).timeout
		highlight_tile(last_path.tile, traveled_material)
		highlight_tile(location.tile, highlight_material)
		last_path = location

func highlight_tile(tile: Vector2i, mat: Material):
	print("replaying over tile " + str(tile))
	var floor_tile = floors.floors_indexed[tile.x + tile.y * maze_data.size.x]
	var mesh: MeshInstance3D = floor_tile.find_child("MeshInstance3D")
	mesh.material_overlay = mat
