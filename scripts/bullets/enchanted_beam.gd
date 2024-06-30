extends Area2D

@export var direction: Vector2
@export var speed: float


# Called when the node enters the scene tree for the first time.
func _ready():
	rotation = PI / 4 + Vector2(0, 0).angle_to_point(direction)
	direction = direction.normalized()
	var color = modulate
	modulate.a = 0
	create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_QUAD).tween_property(
		self, "modulate", color, 0.15
	)
	$OutScreen.screen_exited.connect(queue_free)


# 是否被反弹
var is_bound = false


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	position += delta * speed * direction


func _on_out_screen_screen_exited():
	queue_free()


func _on_area_entered(area):
	# 防止重复伤害
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
	#elif area.name == 盾
	#is_bound = true
	#speed = -speed
