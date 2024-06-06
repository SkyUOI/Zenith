extends Area2D

@export var direction : Vector2
@export var speed : float
# Called when the node enters the scene tree for the first time.
func _ready(): 
	rotation = PI / 4 + Vector2(0, 0).angle_to_point(direction)
	direction = direction.normalized()
	set_process(0)
	$Start.timeout.connect(start)
	$Out_Screen.screen_exited.connect(queue_free)
	
func start():
	set_process(1)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	print(position)
	position += delta * speed * direction 
		

