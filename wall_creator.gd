extends Node3D
class_name WallCreator

@export var maze_data : MazeConfig

## Must implement Node3D and be a prefab. how can I enforce this, what the fuck
@export var maze_wall : Resource

var walls: Array[TileEdge]

class TileEdge:
	var tileA : Vector2i 
	var tileB : Vector2i
	
	func _init(a: Vector2i, b: Vector2i):
		tileA = a
		tileB = b

	static func to_3d_from_2d(flat: Vector2i):
		return Vector3(flat.x, 0, flat.y)

	func spawn_wall_at_edge(maze_wall: Resource, parent: Node):
		var worldA = to_3d_from_2d(tileA)
		var worldB = to_3d_from_2d(tileB)
		
		var center = (worldA + worldB) / 2
		var forward = (worldA - worldB).normalized()
		var new_wall : Wall = maze_wall.instantiate() # explicit cast :(
		new_wall.transform.origin = center
		new_wall.transform.basis = Basis.looking_at(forward)
		parent.add_child(new_wall)

# Called when the node enters the scene tree for the first time.
func _ready():
	walls = randomized_dfs_walls(maze_data.size, RandomNumberGenerator.new())
	for wall in walls:
		wall.spawn_wall_at_edge(maze_wall, self)

	
func random_walls(size: Vector2i, rng: RandomNumberGenerator):
	
	var tmp_walls: Array[TileEdge]
	for x in maze_data.size.x - 1:
		for y in maze_data.size.y:
			if rng.randi_range(0, 1) >= 1 : 
				continue
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x + 1, y)
			))
			
	for x in maze_data.size.x:
		for y in maze_data.size.y - 1:
			if rng.randi_range(0, 1) >= 1 : 
				continue
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x, y + 1)
			))
	return tmp_walls;



static var neighbors = [
	Vector2i.RIGHT,
	Vector2i.LEFT,
	Vector2i.UP,
	Vector2i.DOWN,
]

class Reachability:
	var reachable_dicts : Dictionary
	var size : Vector2i
	var edges : Array[TileEdge]
	func _init(size: Vector2i):
		self.size = size
		reachable_dicts = {}
	
	func in_bounds(position: Vector2i):
		if(position.x <= 0 || position.y <= 0):
			return false
		if(position.x >= size.x || position.y >= size.y):
			return false
		return true
	
	func reachable(position: Vector2i):
		if(position in reachable_dicts):
			return true
		return false
	
	func reach_between(a : Vector2i, b: Vector2i):
		reachable_dicts[a] = null
		reachable_dicts[b] = null
		edges.append(TileEdge.new(a, b))


func randomized_dfs_walls(size: Vector2i, rng: RandomNumberGenerator):
	var reachable := Reachability.new(size)
	var origin := Vector2i(rng.randi_range(0, size.x - 1), rng.randi_range(0, size.y - 1))
	
	internal_randomize_dfs_walls(reachable, origin)
	
	var all_walls = all_edges(size)
	
	var new_walls : Array[TileEdge] = []
	for wall in all_walls:
		if wall in reachable.edges:
			continue
		new_walls.append(wall)
	
	return new_walls
	
	

func internal_randomize_dfs_walls(reach: Reachability, cell: Vector2i):
	for i in 4:
		var next_search = cell + neighbors[i]
		if(!reach.in_bounds(next_search) || reach.reachable(next_search)):
			continue
		reach.reach_between(cell, next_search)
		internal_randomize_dfs_walls(reach, next_search)
	
		

func invert_edges(size: Vector2i, edges: Array[TileEdge]):
	var tmp_walls: Array[TileEdge]
	for x in maze_data.size.x - 1:
		for y in maze_data.size.y:
			var new_edge = TileEdge.new(
				Vector2i(x, y),
				Vector2i(x + 1, y)
			)
			if new_edge in edges:
				continue
			tmp_walls.append(new_edge)
			
	for x in maze_data.size.x:
		for y in maze_data.size.y - 1:
			var new_edge = TileEdge.new(
				Vector2i(x, y),
				Vector2i(x, y + 1)
			)
			if new_edge in edges:
				continue
			tmp_walls.append(new_edge)
	return tmp_walls;

func all_edges(size: Vector2i):
	var tmp_walls: Array[TileEdge]
	for x in maze_data.size.x - 1:
		for y in maze_data.size.y:
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x + 1, y)
			))
			
	for x in maze_data.size.x:
		for y in maze_data.size.y - 1:
			tmp_walls.append(TileEdge.new(
				Vector2i(x, y),
				Vector2i(x, y + 1)
			))
	return tmp_walls;








