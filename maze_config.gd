extends Node
class_name MazeConfig       
   

@export var size := Vector2i(10, 10)

@export var entry := Vector2i(0, 0)
@export var exit := size

static func to_3d_from_2d(flat: Vector2i):
	return Vector3(flat.x, 0, flat.y)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
