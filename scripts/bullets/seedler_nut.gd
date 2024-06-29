extends Area2D

@export var thorns: PackedScene


# 启动投掷动作
# 大小最小, 会下落, 撞到地面炸开
# pos: 初始
func throw():
	area_entered.connect(throwShotBeam)
	$OutScreen.screen_exited.connect(throwShotBeam.bind(Area2D.new()))
	create_tween().tween_property(self, "rotation", rotation + 10.0 * TAU, 5)
	var move = create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_QUAD)
	move.tween_interval(0.5)
	move.tween_property(self, "position", position + Vector2(0, 6500), 3)


func throwShotBeam(area: Area2D):
	#if area.name != "down"
	#return
	const times = 4
	var beams: Array[Area2D] = []
	var velocity = [
		Vector2(-50, -650),
		Vector2(-30, -750),
		Vector2(50, -650),
		Vector2(30, -750),
	]
	for i in range(times):
		beams.push_back(thorns.instantiate())
		beams[i].is_fall = true
		beams[i].position = position
		beams[i].velocity = velocity[i]
	for beam in beams:
		get_parent().add_child(beam)
	queue_free()


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
	tween.tween_callback(burstShotBeam)
	tween.tween_callback(queue_free)


func burstShotBeam():
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
	throw()


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
