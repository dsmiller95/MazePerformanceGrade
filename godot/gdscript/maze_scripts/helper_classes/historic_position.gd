extends Object

class_name HistoricPosition

var tile: Vector2i
var time_ms: int

func _init(tile: Vector2i, time: int = Time.get_ticks_msec()):
	self.tile = tile
	time_ms = time
