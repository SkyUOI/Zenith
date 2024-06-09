extends Area2D

@export var direction: Vector2
@export var speed: float


# Called when the node enters the scene tree for the first time.
func _ready():
	rotation = PI / 4 + Vector2(0, 0).angle_to_point(direction)
	direction = direction.normalized()
	$OutScreen.screen_exited.connect(queue_free)


func start():
	set_process(1)
	$Start.start()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	position += delta * speed * direction * (1.0 if $Start.time_left == 0 else (1.0 / 6.0))


func _on_out_screen_screen_exited():
	queue_free()


# 是否攻击过player
var is_used = false


func _on_area_entered(area):
	# 防止重复伤害
	if area.name == "Player" && !is_used:
		area.get_node("..").hit(10)
		is_used = true
