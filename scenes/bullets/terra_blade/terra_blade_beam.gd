extends Area2D

var tween = create_tween()


func straight_move(
	from: Vector2, to: Vector2, time: float, ease: Tween.EaseType, trans: Tween.TransitionType
):
	rotation = (to - from).angle()
	position = from
	tween.tween_property(self, "position", to, time).set_ease(ease).set_trans(trans)


func _on_out_screen_screen_exited():
	queue_free()

#func _ready():
#straight_move(Vector2(100, 100), Vector2(500, 500), 2, Tween.EASE_IN, Tween.TRANS_CUBIC)
