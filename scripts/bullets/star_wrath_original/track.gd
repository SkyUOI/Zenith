extends Node2D

var tween_obj
var color = Color8(104, 43, 110)
var a = 255
var rect_width = 100


func init(pos: Vector2):
	global_position = pos
	show()
	tween_obj = create_tween().set_loops()
	tween_obj.tween_callback(self.general_hide).set_delay(0.01)


func general_hide():
	if a <= 0:
		tween_obj.kill()
		queue_free()
		return
	a -= 1
	queue_redraw()


func _draw() -> void:
	var real_color = color
	real_color.a = a
	var y_sz = get_viewport_rect().size.y
	draw_rect(
		Rect2(
			self.global_position.x,
			self.global_position.y,
			self.global_position.x + rect_width,
			self.global_position.y + y_sz
		),
		real_color
	)


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	hide()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
