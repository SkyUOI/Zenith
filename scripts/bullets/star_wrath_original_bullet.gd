extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

var direct = Vector2(0,0)
var speed

func init(x_pos, y_pos):
	#print("new bullet")
	self.position.x = x_pos
	self.position.y = y_pos
	self.speed = randi_range(300,400)
	show()
	direct.x = randf_range(-30, -100)
	direct.y = randf_range(30, 100)
	self.transform = self.transform.rotated_local(atan(direct.y / direct.x))

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	self.position += direct.normalized() * delta * speed
	#print(self.position)
