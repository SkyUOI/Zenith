extends StarWrath

@export var star_wrath_origin: PackedScene
var operation_idx = 0
# func(): self.fall_star_process(),
var operations = [func(): self.beam_shoot1(), func(): self.leave()]
@onready var animation_player = $AnimationPlayer
@onready var star_wrath = $StarWrath
var beam_scene: PackedScene = preload("res://scenes/bullets/star_wrath/laser_beam.tscn")


func next_operation():
	if operation_idx >= operations.size():
		$"..".attack_finished.emit()
		return
	var tmp = operation_idx
	operation_idx += 1
	operations[tmp].call()


func fall_star_process():
	for i in range(10):
		self.fall_star()
		await get_tree().create_timer(randf_range(1.0, 2.0)).timeout
	next_operation()


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
