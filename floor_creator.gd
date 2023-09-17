extends Node3D
class_name FloorGenerator


@export var floor_prefab : Resource
@export var maze_data: MazeConfig

# Called when the node enters the scene tree for the first time.
func _ready():
	for x in maze_data.size.x:
		for y in maze_data.size.y:
			var new_node : Node3D = floor_prefab.instantiate()
			new_node.transform.origin = Vector3(x, 0, y)
			add_child(new_node)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
