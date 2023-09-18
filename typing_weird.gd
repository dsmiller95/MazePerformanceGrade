extends Node

@export var hit_all_errors := false

class GenericData:
	var theta: float
	var length: int

func _ready():
	array_generic_return_int_with_assign([1, 2, 3])
	
	#WEIRD no try/catch. must bubble errors up all the way every time?
	if(!hit_all_errors): return
	min_of_vects_fails_runtime(Vector2i.DOWN, Vector2i.UP)
	array_generic_return_int_fails_runtime([1, 2, 3])
	array_generic_return_int_with_cast_fails_runtime([1, 2, 3])
	array_generic_return_generic_data_fails_runtime([GenericData.new(), GenericData.new(), GenericData.new()])
	

########## Type hint failure on global utils ###########
#vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv#
func min_of_vects_fails_runtime(a: Vector2i, b: Vector2i):
	#WEIRD runtime: Invalid type in utility function 'min'. Cannot convert argument 1 from Vector2i to float.
	var min_val = min(a, b)

################ no real Generic arrays ################
#vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv#

func array_generic_return_int_with_assign(input: Array[int]) -> Array[int]:
	# workaround via 2nd allocation
	var random_selected: Array = select_random(input, RandomNumberGenerator.new())
	var random_selected_typed: Array[int]
	random_selected_typed.assign(random_selected)
	return random_selected_typed

func array_generic_return_int_fails_runtime(input: Array[int]) -> Array[int]:
	var random_selected := select_random(input, RandomNumberGenerator.new())
	#WEIRD runtime: Trying to return an array of type "Array" where expected return type is "Array[int]".
	return random_selected
	
func array_generic_return_int_with_cast_fails_runtime(input: Array[int]) -> Array[int]:
	#WEIRD runtime: Trying to assign an array of type "Array" to a variable of type "Array[int]".
	var random_selected: Array[int] = select_random(input, RandomNumberGenerator.new()) as Array[int]
	return random_selected
	

func array_generic_return_generic_data_fails_runtime(input: Array[GenericData]) -> Array[GenericData]:
	var random_selected := select_random(input, RandomNumberGenerator.new())
	#WEIRD runtime: Trying to return an array of type "Array" where expected return type is "Array[GenericData]".
	return random_selected
	
func select_random(input: Array, rng: RandomNumberGenerator) -> Array:
	var result: Array = []
	for a in input:
		if (rng.randi_range(0, 1) >= 1):
			continue
		result.append(a)
	return result
