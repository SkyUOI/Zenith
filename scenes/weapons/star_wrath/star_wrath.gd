extends StarWrath

@export var star_wrath_origin: PackedScene
var operation_idx = 0
var operations = [
	func(): self.fall_star_process(),
	func(): self.beam_shoot1(),
	func(): self.fall_star_explode(),
	func(): five_surrounding_points(),
	func(): self.leave()
]
@onready var animation_player = $AnimationPlayer
@onready var star_wrath = $StarWrath
var beam_scene: PackedScene = preload("res://scenes/bullets/star_wrath/laser_beam.tscn")
var point_scene: PackedScene = preload("res://scenes/bullets/star_wrath/point.tscn")


func next_operation():
	if operation_idx >= operations.size():
		$"..".attack_finished.emit()
		return
	var tmp = operation_idx
	operation_idx += 1
	operations[tmp].call()


func fall_star_process():
	for i in range(10):
		self.fall_star(false)
		await get_tree().create_timer(randf_range(1.0, 2.0)).timeout
	next_operation()


func fall_star_explode():
	for i in range(4):
		self.fall_star(true)
		await get_tree().create_timer(randf_range(5.0, 6.0)).timeout
	next_operation()


func five_surrounding_points():
	var points = []
	for i in range(5):
		var point = point_scene.instantiate()
		points.push_back(point)


func beam_shoot1():
	var beam = beam_scene.instantiate()
	self.add_child(beam)
	beam.init(Vector2(200, 450))
	beam.set_is_casting(true)
	beam.set_direct(beam.to_local(Vector2(300, 450)))
	await get_tree().create_timer(3.0).timeout
	beam.set_is_casting(false)


func leave():
	await self.get_tree().create_timer(0.3).timeout
	var tween = create_tween()
	tween.tween_property(star_wrath, "global_position", Vector2(-100.0, -100), 2)
	tween.tween_callback($"..".general_hide.bind())
	next_operation()


func laser():
	pass
