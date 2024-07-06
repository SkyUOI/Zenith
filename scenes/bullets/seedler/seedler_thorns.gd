extends Area2D

# 是否坠落
var is_fall: bool
# 速度
var velocity: Vector2


func _ready():
	$OutScreen.screen_exited.connect(queue_free)


func _physics_process(delta):
	position += velocity * delta
	velocity += (1 if is_fall else 0) * Vector2(0, 1300) * delta


func _on_area_entered(area):
	# 防止重复伤害
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
	#elif area.name == 盾
	# $CollisionShape2D.set_deferred("disabled", true)
	# 有盾就反弹
	# velocity = -velocity
