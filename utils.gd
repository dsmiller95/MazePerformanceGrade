class_name Utils


static func partswise(a: Vector2i, b: Vector2i, f: Callable) -> Vector2i:
	var x: int = f.call(a.x, b.x)
	var y: int = f.call(a.y, b.y)
	return Vector2i(x, y)

static var min = func(a, b): return min(a, b)
static var max = func(a, b): return max(a, b)

static func shuffle_naive(arr: Array, rng: RandomNumberGenerator):
	for i in arr.size():
		var swap = rng.randi_range(i, arr.size() - 1)
		var tmp = arr[swap]
		arr[swap] = arr[i]
		arr[i] = tmp
