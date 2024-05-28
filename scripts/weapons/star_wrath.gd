extends Sprite2D

var start_flag = true

# Called when the node enters the scene tree for the first time.
func _ready():
	pass

func wave():
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !start_flag:
		return
	self.transform = self.transform.rotated_local(0.1 * delta)

func start():
	start_flag = true
