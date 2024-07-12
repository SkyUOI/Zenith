extends Area2D

const WIDTH = 8
@export var thorns: PackedScene
var alpha: float
var line1_from: Vector2
var line1_to: Vector2
var line2_from: Vector2
var line2_to: Vector2


# 启动投掷动作
# 大小最小, 会下落, 撞到地面炸开
# pos: 初始
func throw(pos: Vector2):
	position = pos
	area_entered.connect(throw_shot_beam)
	$OutScreen.screen_exited.connect(throw_shot_beam.bind(Area2D.new()))
	create_tween().tween_property(self, "rotation", rotation + 10.0 * TAU, 5)
	var move = create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_QUAD)
	move.tween_interval(0.5)
	move.tween_property(self, "position", position + Vector2(0, 6500), 3)


func throw_shot_beam(_area: Area2D):
	#if area.name != "down"
	#return
	const TIMES = 4
	var beams: Array[Area2D] = []
	var velocity = [
		Vector2(-50, -650),
		Vector2(-30, -750),
		Vector2(50, -650),
		Vector2(30, -750),
	]
	for i in range(TIMES):
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
	const TIME = 0.8
	# TAU / s
	const SPEED = 1.5 * TAU
	position = pos
	scale = Vector2(1.5, 1.5)
	var tween = create_tween()
	tween.tween_property(self, "rotation", rotation + SPEED * TIME, TIME)
	tween.tween_callback(burst_shot_beam)
	tween.tween_callback(queue_free)


func burst_shot_beam():
	const TIMES = 15
	var beams: Array[Area2D] = []
	for i in range(TIMES):
		beams.push_back(thorns.instantiate())
		beams[i].is_fall = false
		beams[i].position = position
		var rad = TAU / TIMES * i + (TAU / 12 if i % 2 == 0 else -TAU / 12)
		beams[i].rotation = rad
		beams[i].velocity = Vector2(sin(rad), cos(rad)) * 800
	for beam in beams:
		get_parent().add_child(beam)


func change_alpha(x: float):
	alpha = x
	queue_redraw()


# 启动超级爆炸
# 最大, 原地旋转
func super_burst(pos: Vector2):
	const TIME = 1.2
	const WAIT_TIME = 0.65
	# TAU / s
	const SPEED = 1.5 * TAU
	position = pos
	rotation = randf_range(0, TAU)
	scale = Vector2(2, 2)
	line1_from = 70 * Vector2(cos(TAU / 12 * 1), sin(TAU / 12 * 1))
	line1_to = 70 * Vector2(cos(TAU / 12 * 7), sin(TAU / 12 * 7))
	line2_from = 70 * Vector2(cos(TAU / 12 * 4), sin(TAU / 12 * 4))
	line2_to = 70 * Vector2(cos(TAU / 12 * 10), sin(TAU / 12 * 10))
	var line = create_tween()
	line.tween_interval(TIME / 4)
	line.tween_method(change_alpha, 0.01, 0.7, TIME / 2)
	line.tween_method(change_alpha, 0.7, 0.01, TIME / 4)
	var tween = create_tween()
	tween.tween_property($Sprite2D, "rotation", $Sprite2D.rotation + SPEED * TIME, TIME)
	tween.tween_callback(super_burst_shot_beam)
	tween.tween_callback($Sprite2D.queue_free)
	tween.tween_interval(WAIT_TIME)
	tween.tween_callback(queue_free)


func _draw():
	var yellow = Color(Color.GOLD, alpha)
	draw_line(line1_from, line1_to, yellow, WIDTH)
	draw_line(line2_from, line2_to, yellow, WIDTH)


func super_burst_shot_beam():
	const TIMES = 12
	const NUM = 8
	for j in range(NUM):
		var beams: Array[Area2D] = []
		for i in range(TIMES):
			beams.push_back(thorns.instantiate())
			beams[i].is_fall = false
			beams[i].position = position
			var rad = TAU / TIMES * i + rotation
			if i % 3 == 0:
				rad += PI / 10
			elif i % 3 == 2:
				rad -= PI / 10
			beams[i].velocity = Vector2(cos(rad), sin(rad)) * 1600
		for beam in beams:
			get_parent().add_child(beam)
		await get_tree().create_timer(0.05).timeout


func _ready():
	$OutScreen.screen_exited.connect(queue_free)
	if get_tree().current_scene == self:
		throw(position)


func _on_area_entered(area):
	# 防止重复伤害
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
