extends StarWrathBullet

@export var track: PackedScene
var tween
var block_color: Color = Color8(255, 255, 255, 255)
var littlestar_scene = preload("res://scenes/bullets/star_wrath/little_star.tscn")


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	hide()


func explode_effect(callback):
	for i in range(3):
		var direct = PI / 12
		for j in range(10):
			var littlestar = littlestar_scene.instantiate()
			littlestar.shoot(direct * j)
			add_child(littlestar)
		await get_tree().create_timer(0.1).timeout
	callback.call()


func _on_killer_screen_exited() -> void:
	await get_tree().create_timer(0.5).timeout
	if self.explode:
		# 爆炸
		self.explode_effect(queue_free)
	else:
		queue_free()
