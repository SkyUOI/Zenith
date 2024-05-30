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
	#print("star ready")
	#new_bullet()
	if get_tree().current_scene == self:
		$Fight_Timer.start()

func wave():
	self.state = States.Wave1
	new_bullet()

@export var star_wrath_bullet:PackedScene
@export var BULLETS_NUM:int

func new_bullet():
	for i in range(BULLETS_NUM):
		var star = star_wrath_bullet.instantiate()
		var sz = get_viewport_rect().size.x
		star.init(sz, 0)
		$"..".add_child.call_deferred(star)

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

func _on_fight_t_imer_timeout():
	new_bullet()


func _on_killer_screen_exited():
	queue_free()
