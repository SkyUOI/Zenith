extends Area2D

var tween = create_tween()


func straight_move(
	from: Vector2, to: Vector2, time: float, ease_: Tween.EaseType, trans: Tween.TransitionType
):
	rotation = (to - from).angle()
	position = from
	tween.tween_property(self, "position", to, time).set_ease(ease_).set_trans(trans)

func falling_move(
	from: Vector2, velocity: Vector2, g: float
):
	var vx = velocity.x
	var vy = velocity.y
	rotation = Vector2(vx, vy).angle()
	var f = func(t):
		var x = vx * t
		var y = vy * t + g * t * t / 2.0
		position = from + Vector2(x, y)
		rotation -= Vector2.DOWN.angle_to(Vector2(cos(rotation), sin(rotation))) / 70
	tween.tween_method(f, 0.0, 10.0, 10)
	

func _on_out_screen_screen_exited():
	queue_free()

#func _ready():
#straight_move(Vector2(100, 100), Vector2(500, 500), 2, Tween.EASE_IN, Tween.TRANS_CUBIC)
