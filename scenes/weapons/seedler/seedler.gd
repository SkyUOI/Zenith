extends Sprite2D

const TIMES = 5
@onready var path = $"../Path2D"
@onready var follow = $"../Path2D/PathFollow2D"


func move(length: float) -> void:
	follow.progress = length
	position = follow.global_position


func _ready() -> void:
	if get_tree().current_scene != $"..":
		hide()
		return
	rotation = 0
	follow.loop = 1
	var start = 0
	var end = path.curve.get_baked_length()
	var mid = end / 2
	var movement = create_tween().set_loops(TIMES).set_ease(Tween.EASE_IN).set_trans(
		Tween.TRANS_SINE
	)
	movement.tween_method(move, start, mid, 1)
	movement.tween_method(move, mid, end, 1)
	var rotate_animation = create_tween().set_loops(TIMES)
	rotate_animation.tween_property(self, "rotation", TAU * 5, 2)
	rotate_animation.tween_callback(func(): rotation = 0)
