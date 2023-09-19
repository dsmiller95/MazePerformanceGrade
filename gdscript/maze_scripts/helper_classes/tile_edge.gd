extends Object


class_name TileEdge

var tileA : Vector2i 
var tileB : Vector2i

func _init(a: Vector2i, b: Vector2i):
	tileA = a
	tileB = b

static func to_3d_from_2d(flat: Vector2i):
	return Vector3(flat.x, 0, flat.y)

func get_stable_id_in_domain(size: Vector2i) -> int:
	var min_vec = Utils.partswise(tileA, tileB, Utils.min)
	var max_vec = Utils.partswise(tileA, tileB, Utils.max)
	return min_vec.x + min_vec.y * size.x + max_vec.x * size.x * size.y + max_vec.y + size.x * size.y * size.x

func spawn_wall_at_edge(maze_wall: Resource, parent: Node):
	var worldA = to_3d_from_2d(tileA)
	var worldB = to_3d_from_2d(tileB)

	var center = (worldA + worldB) / 2
	var forward = (worldA - worldB).normalized()
	var new_wall : Node3D = maze_wall.instantiate() # explicit cast :(
	new_wall.transform.origin = center
	new_wall.transform.basis = Basis.looking_at(forward)
	parent.add_child(new_wall)
