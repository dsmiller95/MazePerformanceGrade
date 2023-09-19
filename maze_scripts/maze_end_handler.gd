extends Node

@export var maze_cam: Camera3D
@export var replay: MazeReplay
@export var score_text: RichTextLabel

@export var maze_config: MazeConfig
@export var walls: WallCreator

@export var player_tracker: PathHistory

@export var maze_solvers: Array[Solver]

@export var replay_total_time_seconds: float = 5

func _on_end_marker_entered(area):
	play_end_game()
	
var is_end_game_running := false

func play_end_game():
	if (is_end_game_running): return;
	is_end_game_running = true
	
	maze_cam.make_current()
	await replay.begin_path_replay(player_tracker.path_history, replay_total_time_seconds * 1000)
	score_text.append_text("player: " + str(player_tracker.path_history.size()) + "\n")
	
	for solver in maze_solvers:
		var solution := solver.solve_path(walls.reachable, maze_config.entry, 0, maze_config.exit)
		await replay.begin_path_replay(solution, replay_total_time_seconds * 1000)
		score_text.append_text(solver.name + ": " + str(solution.size()) + "\n")
