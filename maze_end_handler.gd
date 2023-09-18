extends Node

@export var maze_cam: Camera3D
@export var replay: MazeReplay
@export var player_tracker: PathHistory


func _on_end_marker_entered(area):
	maze_cam.make_current()
	replay.begin_path_replay(player_tracker.path_history)
