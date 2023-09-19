extends Object

class_name Reachability


## increment by one turns to the right
static var neighbors := [
	Vector2i.UP,
	Vector2i.RIGHT,
	Vector2i.DOWN,
	Vector2i.LEFT,
]


var reachable_dicts : Dictionary
var size : Vector2i
var edges : Array[TileEdge]
func _init(size: Vector2i):
	self.size = size
	reachable_dicts = {}

func in_bounds(position: Vector2i):
	if(position.x < 0 || position.y < 0):
		return false
	if(position.x >= size.x || position.y >= size.y):
		return false
	return true

func reachable(position: Vector2i):
	if(_to_index(position) in reachable_dicts):
		return true
	return false

func assert_fully_reachable():
	for x in size.x:
		for y in size.y:
			var pos = Vector2i(x, y)
			if(! reachable(pos)):
				print("ERROR: cannot reach " + str(pos))

func reach_between(already_reached: Vector2i, newly_reached: Vector2i):
	if _to_index(already_reached) not in reachable_dicts:
		print("ERROR: trying to reach from unreached cell")
	if _to_index(newly_reached) in reachable_dicts:
		print("ERROR: reaching into an already reached cell")
	reachable_dicts[_to_index(newly_reached)] = null
	edges.append(TileEdge.new(already_reached, newly_reached))

func _to_index(a: Vector2i) -> int:
	return a.x + a.y * (size.x + 1)

func edge_id(edge: TileEdge) -> int:
	return edge.get_stable_id_in_domain(size)

func all_edge_ids() -> Array[int]:
	var result : Array[int]
	for edge in edges:
		result.append(edge_id(edge))
	return result

func get_walls() -> Array[TileEdge]:
	var except_walls = Utils.except_by(
		all_edges(size), 
		edges,
		func (edge): return edge_id(edge)
		)
	
	var except_walls_typed: Array[TileEdge]
	except_walls_typed.assign(except_walls)
	
	return except_walls_typed

static func all_edges(size: Vector2i):
	var tmp_walls: Array[TileEdge]
	for x in size.x - 1:
		for y in size.y:
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x + 1, y)
			))
			
	for x in size.x:
		for y in size.y - 1:
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x, y + 1)
			))
	return tmp_walls;
