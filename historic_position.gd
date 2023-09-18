extends Object

class_name HistoricPosition

var tile: Vector2i
var time_ms: int

func _init(tile: Vector2i):
	self.tile = tile
	time_ms = Time.get_ticks_msec()
