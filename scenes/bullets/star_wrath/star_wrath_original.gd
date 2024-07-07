extends StarWrathBullet

@export var track: PackedScene
var tween
var block_color: Color = Color8(255, 255, 255, 255)


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	hide()


func _on_killer_screen_exited() -> void:
	await get_tree().create_timer(0.5).timeout
	print("Killing")
	queue_free()
