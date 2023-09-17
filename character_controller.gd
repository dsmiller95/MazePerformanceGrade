extends Node3D
class_name CharacterController

@export var speed : float = 1

func get_movement_inputs() -> Vector3:
	var all_inputs := Vector3(0, 0, 0)
	if(Input.is_action_pressed("move_right")):
		all_inputs += Vector3.RIGHT
	if(Input.is_action_pressed("move_left")):
		all_inputs += Vector3.LEFT
	if(Input.is_action_pressed("move_forward")):
		all_inputs += Vector3.FORWARD
	if(Input.is_action_pressed("move_back")):
		all_inputs += Vector3.BACK
	return all_inputs

func _physics_process(delta: float):
	var inputs := get_movement_inputs()
	inputs = inputs.normalized()
	position += inputs * speed * delta
	
