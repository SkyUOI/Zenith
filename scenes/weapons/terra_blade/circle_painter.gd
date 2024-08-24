extends Node2D

var rad: float
var center: Vector2
var r: float


func _draw():
	draw_arc(center, r, -TAU / 4, -TAU / 4 + rad, 50, Color(0, 1, 0), 30, true)


func draw(rad_: float, center_: Vector2, r_: float):
	rad = rad_
	center = center_
	r = r_
	queue_redraw()


func draw2(r_: float, rad_: float, center_: Vector2):
	rad = rad_
	center = center_
	r = r_
	queue_redraw()
