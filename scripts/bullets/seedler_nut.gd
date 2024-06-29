extends Area2D

@export var thorns: PackedScene


# 启动投掷动作
# 大小最小, 会下落, 撞到地面炸开
# pos: 初始
func throw():
	pass


# 启动爆炸动作
# 大小中等, 原地旋转, 爆炸
func burst(pos: Vector2):
	const time = 0.8
	# TAU / s
	const speed = 1.5 * TAU
	position = pos
	scale = Vector2(1.5, 1.5)
	var tween = create_tween()
	tween.tween_property(self, "rotation", rotation + speed * time, time)
	tween.tween_callback(burstShotBeam.bind(pos))
	tween.tween_callback(queue_free)


func burstShotBeam(pos: Vector2):
	const times = 15
	var beams: Array[Area2D] = []
	for i in range(times):
		beams.push_back(thorns.instantiate())
		beams[i].is_fall = false
		beams[i].position = position
		var rad = TAU / times * i + (TAU / 12 if i % 2 == 0 else -TAU / 12)
		beams[i].velocity = Vector2(sin(rad), cos(rad)) * 800
	for beam in beams:
		get_parent().add_child(beam)


func _ready():
	$OutScreen.screen_exited.connect(queue_free)
	burst(Vector2(300, 300))


func _process(delta):
	pass


func _on_area_entered(area):
	# thornsCreate()
	# 防止重复伤害
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
