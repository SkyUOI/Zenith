extends Sprite2D

@onready var path = $"../Path2D"
@onready var follow = $"../Path2D/PathFollow2D"

func move(length : float) -> void:
	follow.progress = length
	position = follow.global_position
	
func start() -> void:
	rotation = 0
	follow.loop = 1
	var start_val = 0
	var end = path.curve.get_baked_length()
	var mid = end / 2
	var movement = create_tween().set_loops(times).set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_SINE)
	movement.tween_method(move, start_val, mid, 1)
	movement.tween_method(move, mid, end, 1)
	var rotate_tween = create_tween().set_loops(times)
	rotate_tween.tween_property(self, "rotation", TAU * 5, 2)
	rotate_tween.tween_callback(func():rotation = 0)	
	
const times = 5	

func _ready() -> void:
	if get_tree().current_scene == self:
		show()
		start()
	else:
		hide()
