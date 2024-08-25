extends Sprite2D

const SPEED: float = 90
var direct


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(false)


func shoot(direction: Vector2):
	assert(direction.length() > 0)
	self.direct = direction.normalized()
	set_process(true)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	self.position += delta * SPEED * direct


func _on_visible_on_screen_notifier_2d_screen_exited() -> void:
	queue_free()
