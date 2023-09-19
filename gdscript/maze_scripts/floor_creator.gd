extends Node3D
class_name FloorGenerator


@export var floor_prefab : Resource
@export var maze_data: MazeConfig

var floors_indexed: Array[Node3D] = []

# Called when the node enters the scene tree for the first time.
func _ready():
	var size = maze_data.size
	floors_indexed.resize(size.x * size.y)
	for x in size.x:
		for y in size.y:
			var new_node : Node3D = floor_prefab.instantiate()
			new_node.transform.origin = Vector3(x, 0, y)
			add_child(new_node)
			floors_indexed[x + y * size.x] = new_node
