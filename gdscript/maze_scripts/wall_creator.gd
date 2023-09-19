extends Node3D
class_name WallCreator

@export var maze_data : MazeConfig

## Must implement Node3D and be a prefab. how can I enforce this, what the fuck
#WEIRD I can't enforce a reference to a resource must reference a Node3D scene? or another specific type?
@export var maze_wall : Resource

var walls: Array[TileEdge]

var reachable: Reachability

# Called when the node enters the scene tree for the first time.
func _ready():
	reachable = randomized_dfs_walls(maze_data.size, RandomNumberGenerator.new())
	walls = reachable.get_walls()
	for wall in walls:
		wall.spawn_wall_at_edge(maze_wall, self)


func randomized_dfs_walls(size: Vector2i, rng: RandomNumberGenerator):
	var reachable_tmp = Reachability.new(size)
	var origin := Vector2i(rng.randi_range(0, size.x - 1), rng.randi_range(0, size.y - 1))
	
	internal_randomize_dfs_walls(reachable_tmp, origin, rng)
	reachable_tmp.assert_fully_reachable()
	
	return reachable_tmp

func internal_randomize_dfs_walls(reach: Reachability, cell: Vector2i, rng: RandomNumberGenerator):
	var indexes = [0, 1, 2, 3]
	Utils.shuffle_naive(indexes, rng)
	for i in indexes:
		var next_search = cell + Reachability.neighbors[i]
		if(!reach.in_bounds(next_search) || reach.reachable(next_search)):
			continue
		reach.reach_between(cell, next_search)
		internal_randomize_dfs_walls(reach, next_search, rng)








