extends Node3D
class_name WallCreator

@export var maze_data : MazeConfig

## Must implement Node3D and be a prefab. how can I enforce this, what the fuck
@export var maze_wall : Resource

var walls: Array[WallConnection]

class WallConnection:
	var tileA : Vector2i 
	var tileB : Vector2i
	
	func _init(a: Vector2i, b: Vector2i):
		tileA = a
		tileB = b

	static func to_3d_from_2d(flat: Vector2i):
		return Vector3(flat.x, 0, flat.y)

	func spawn_wall(maze_wall: Resource, parent: Node):
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
	walls = random_walls(maze_data.size, RandomNumberGenerator.new())
	for wall in walls:
		wall.spawn_wall(maze_wall, self)

	
func random_walls(size: Vector2i, rng: RandomNumberGenerator):
	
	var tmp_walls: Array[WallConnection]
	for x in maze_data.size.x - 1:
		for y in maze_data.size.y:
			if rng.randi_range(0, 1) >= 1 : 
				continue
			tmp_walls.append(WallConnection.new(
				Vector2i(x, y),
				Vector2i(x + 1, y)
			))
			
	for x in maze_data.size.x:
		for y in maze_data.size.y - 1:
			if rng.randi_range(0, 1) >= 1 : 
				continue
			tmp_walls.append(WallConnection.new(
				Vector2i(x, y),
				Vector2i(x, y + 1)
			))
	return tmp_walls;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
