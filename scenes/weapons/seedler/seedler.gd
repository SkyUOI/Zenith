extends Sprite2D

signal attack_finished
const TIMES = 5
@onready var path = $"../Path2D"
@onready var follow = $"../Path2D/PathFollow2D"
@export var beam: PackedScene


func move(length: float) -> void:
	follow.progress = length
	position = follow.global_position


func shot_beam(x: int, pos: Vector2):
	var new_beam = beam.instantiate()
	get_parent().add_child(new_beam)
	match x:
		1:
			new_beam.throw(pos)
		2:
			new_beam.burst(pos)
		3:
			new_beam.super_burst(pos)


func start() -> void:
	rotation = 0
	follow.loop = 1
	var start_val = 0
	var end = path.curve.get_baked_length()
	var mid = end / 2
	var movement = create_tween().set_loops(TIMES)
	movement.tween_method(move, start_val, mid, 1.25)
	movement.tween_method(move, mid, end, 1.25)
	var exit = create_tween()
	exit.tween_interval(TIMES * 2.5)
	exit.tween_callback(attack_finished.emit)
	exit.tween_callback(queue_free)

	var rotate_tween = create_tween().set_loops(TIMES)
	rotate_tween.tween_property(self, "rotation", TAU * 5, 2.5)
	rotate_tween.tween_callback(func(): rotation = 0)
	var shot = create_tween().set_loops()
	shot.tween_callback(shot_beam.bind(1, Vector2(200, 400)))
	shot.tween_callback(shot_beam.bind(1, Vector2(400, 400)))
	shot.tween_callback(shot_beam.bind(1, Vector2(600, 400)))
	shot.tween_callback(shot_beam.bind(1, Vector2(800, 400)))
	shot.tween_callback(shot_beam.bind(1, Vector2(1000, 400)))
	shot.tween_interval(2)
	shot.tween_callback(shot_beam.bind(2, Vector2(300, 300)))
	shot.tween_interval(1)
	shot.tween_callback(shot_beam.bind(2, Vector2(700, 500)))
	shot.tween_interval(2)
	shot.tween_callback(shot_beam.bind(3, Vector2(500, 500)))
	shot.tween_interval(2)
	shot.tween_callback(shot_beam.bind(3, Vector2(800, 200)))
	shot.tween_interval(2)


func _ready() -> void:
	if get_tree().current_scene == get_parent():
		show()
		start()
	else:
		hide()
