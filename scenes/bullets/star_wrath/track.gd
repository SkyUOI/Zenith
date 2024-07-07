extends Node2D

var tween_obj
var color = Color8(104, 43, 110)
var a = 255
var trans_pos


func init():
	show()
	trans_pos = global_position
	tween_obj = create_tween().set_loops()
	tween_obj.tween_callback(self.general_hide).set_delay(0.01)
	show_behind_parent = true


func general_hide():
	if a <= 0:
		tween_obj.kill()
		queue_free()
		return
	a -= 1
	queue_redraw()


func _draw() -> void:
	draw_set_transform_matrix(Transform2D(global_transform.inverse()))
	var real_color = color
	real_color.a = a / 255.0
	print("draw", real_color)
	var y_sz = get_viewport_rect().size.y
	const WIDTH = 50
	draw_rect(
		Rect2(
			trans_pos.x - WIDTH,
			0,
			WIDTH * 2,
			y_sz,
		),
		real_color
	)


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	hide()
