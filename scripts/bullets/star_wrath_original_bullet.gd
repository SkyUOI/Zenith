extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

var x_speed
var y_speed

func init(x_pos, y_pos):
	self.position.x = x_pos
	self.position.y = y_pos
	show()
	x_speed = randf_range(30, 60)
	y_speed = randf_range(30, 60)
	self.transform = self.transform.rotated_local(atan(x_speed / y_speed))
		
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
