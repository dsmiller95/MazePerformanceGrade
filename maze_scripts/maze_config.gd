extends Node
class_name MazeConfig       
   

@export var size := Vector2i(10, 10)

@export var entry := Vector2i(0, 0)
@export var exit := size

static func to_3d_from_2d(flat: Vector2i):
	return Vector3(flat.x, 0, flat.y)

func in_bounds(position: Vector2i):
	if(position.x < 0 || position.y < 0):
		return false
	if(position.x >= size.x || position.y >= size.y):
		return false
	return true
