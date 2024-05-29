extends Sprite2D

var start_flag = true
enum States{
	Wave1,
	Wave2,
	WaveReSet,
	None
}
var state = States.Wave1

# Called when the node enters the scene tree for the first time.
func _ready():
	pass

func wave():
	self.state = States.Wave1

var star_wrath_bullet = preload("res://scenes/bullets/star_wrath_original_bullet.tscn")

func new_bullet():
	var star = star_wrath_bullet.instantiate()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !start_flag:
		return
	if state == States.None:
		return
	if self.state == States.Wave1:
		self.transform = self.transform.rotated_local(0.1 * delta)
	elif self.state == States.Wave2:
		pass
	elif self.state == States.WaveReSet:
		pass

func start():
	start_flag = true
