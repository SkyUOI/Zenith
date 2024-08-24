extends Node2D

var r: float
var alpha: float
var center: Vector2


func _draw():
	draw_circle(center, r, Color(0, 1, 0, alpha), true, -1, true)


func draw(r_, center_):
	r = r_
	alpha = (270.0 - r_) / 270.0
	center = center_
	queue_redraw()
