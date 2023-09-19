extends Solver
class_name RightHandRule

@export var keep_right := true


func solve_path(maze: Reachability, position: Vector2i, direction: int, target: Vector2i) -> Array[HistoricPosition]:
	var path: Array[HistoricPosition] = []
	var delay_ms := 500
	var current_step := 0
	var max_steps := (maze.size.x * maze.size.y) ** 2
	
	var check_dir = 1 if keep_right else 3
	var search_dir = 3 if keep_right else 1
	
	path.append(HistoricPosition.new(position, current_step * delay_ms))
	while position != target && current_step < max_steps:
		var forward_tile : Vector2i = position + Reachability.neighbors[direction]
		var forward_edge := TileEdge.new(position, forward_tile)
		var can_move_forward = maze.in_bounds(forward_tile) && maze.traversable(forward_edge)
		
		if (can_move_forward):
			position = forward_tile
			# turn to the right by one, to inspect the wall on the right
			direction = (direction + check_dir) % 4
			path.append(HistoricPosition.new(position, current_step * delay_ms))
		else:
			# turn to the left
			direction = (direction + search_dir) % 4
		
		current_step += 1
	
	
	return path


