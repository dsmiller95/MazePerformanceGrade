extends Node3D
class_name WallCreator

@export var maze_data : MazeConfig

## Must implement Node3D and be a prefab. how can I enforce this, what the fuck
@export var maze_wall : Resource

var walls: Array[TileEdge]

var reachable: Reachability

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


		

func randomized_dfs_walls(size: Vector2i, rng: RandomNumberGenerator):
	reachable = Reachability.new(size)
	var origin := Vector2i(rng.randi_range(0, size.x - 1), rng.randi_range(0, size.y - 1))
	
	internal_randomize_dfs_walls(reachable, origin, rng)
	reachable.assert_fully_reachable()
	
	return reachable.get_walls()

func internal_randomize_dfs_walls(reach: Reachability, cell: Vector2i, rng: RandomNumberGenerator):
	var indexes = [0, 1, 2, 3]
	Utils.shuffle_naive(indexes, rng)
	for i in indexes:
		var next_search = cell + Reachability.neighbors[i]
		if(!reach.in_bounds(next_search) || reach.reachable(next_search)):
			continue
		reach.reach_between(cell, next_search)
		internal_randomize_dfs_walls(reach, next_search, rng)








