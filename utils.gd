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

#WEIRD no generic funcs, no generic callable return type
static func except_by(source: Array, except: Array, identity: Callable):
	#WEIRD no generic dict
	#WEIRD no set type
	var except_idents: Dictionary = {}
	for a in except:
		var ident = identity.call(a)
		except_idents[ident] = null
	
	var new_arr = []
	for a in source:
		var ident = identity.call(a)
		if ident in except_idents:
			continue
		new_arr.append(a)
	return new_arr
	
