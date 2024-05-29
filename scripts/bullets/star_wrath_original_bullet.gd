extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

var direct = Vector2(0,0)
var speed = 100

func init(x_pos, y_pos):
	#print("new bullet")
	self.position.x = x_pos
	self.position.y = y_pos
	show()
	direct.x = randf_range(-10, -100)
	direct.y = randf_range(10, 100)
	self.transform = self.transform.rotated_local(atan(direct.x / direct.y))

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	self.position += direct.normalized() * delta * speed
	print(self.position)
