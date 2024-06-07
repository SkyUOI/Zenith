extends Area2D

@export var direction: Vector2
@export var speed: float


# Called when the node enters the scene tree for the first time.
func _ready():
	rotation = PI / 4 + Vector2(0, 0).angle_to_point(direction)
	direction = direction.normalized()
	$Out_Screen.screen_exited.connect(queue_free)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	position += delta * speed * direction * (1.0 if $Start.time_left == 0 else (1.0 / 6.0))


func _on_out_screen_screen_exited():
	queue_free()
