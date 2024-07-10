extends EnchantedSword

signal attack_finished
@export var enchanted_beam: PackedScene
var finished: bool

# 操作序列(也许只是测试用)
var opers: Array[Callable]

# 当前动作对应函数
var now_func: Callable

# 当前是第几个oper(用于opers)
var oper_num: int = -1


func _ready():
	opers = [
		swing_attack.bind(Vector2(330, 330), Vector2(800, 300), 4),
		swing_attack.bind(Vector2(430, 200), Vector2(800, 750), 4),
		swing_attack.bind(Vector2(830, 330), Vector2(300, 400), 4),
		vertical_attack.bind(Vector2(200, 100), Vector2(900, 100), 4),
		vertical_attack.bind(Vector2(200, 500), Vector2(600, 100), 4),
		rotate_attack.bind(Vector2(1000, 100), 6),
		rotate_attack.bind(Vector2(500, 500), 6),
		motionless_attack.bind(Vector2(500, 300), 3),
		motionless_attack.bind(Vector2(600, 400), 3),
		rush_attack.bind(Vector2(130, 230), Vector2(800, 400)),
		rush_attack.bind(Vector2(800, 600), Vector2(200, 200)),
	]
	if get_tree().current_scene == self:
		exit()
	else:
		hide()


# fight调用这个
# 开始
func start():
	show()
	exit()


# 完成某一动作
func exit():
	next_oper()


# 前往下一个动作
func next_oper():
	oper_num += 1
	if oper_num >= opers.size():
		attack_finished.emit()
		return
	opers[oper_num].call()


func is_even(x: int) -> bool:
	return x % 2 == 0


# 角度对应向量
func rad_to_vector(rad: float) -> Vector2:
	return Vector2(cos(rad), sin(rad))


# 发射附魔光束
func swing_shot_beam(start_pos: Vector2, end_pos: Vector2):
	var beam = enchanted_beam.instantiate()
	beam.direction = end_pos - start_pos
	beam.position = start_pos
	beam.speed = 750
	get_parent().add_child(beam)


# 启动挥舞动作
# point: 挥舞路径中点
# to: 目标点
func swing_attack(point: Vector2, to: Vector2, tim: int):
	const MOVE_TIME = 0.4
	const PRE_MOVE_TIME = 0.5
	const WAIT_TIME = 0.05
	var to_target_rad = point.angle_to_point(to)
	var swing_odd_rad = to_target_rad + (-PI * 2 / 3)
	var swing_even_rad = to_target_rad + (PI * 1.5 / 3)
	var swing_odd_position = point + 170 * rad_to_vector(to_target_rad + (-PI / 2))
	var swing_even_position = point + 170 * rad_to_vector(to_target_rad + PI / 2)
	var swing = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC)
	swing.tween_property(self, "position", point, PRE_MOVE_TIME)
	swing.parallel().tween_property(self, "rotation", to_target_rad, PRE_MOVE_TIME)
	swing.tween_interval(WAIT_TIME)
	for i in range(tim):
		var even = is_even(i)
		swing.tween_property(
			self, "position", swing_even_position if even else swing_odd_position, MOVE_TIME
		)
		swing.parallel().tween_property(
			self, "rotation", swing_even_rad if even else swing_odd_rad, MOVE_TIME
		)
		swing.tween_interval(WAIT_TIME)
	swing.tween_callback(exit)

	var vector = swing_odd_position - swing_even_position
	var shot = create_tween()
	shot.tween_interval(PRE_MOVE_TIME + WAIT_TIME)
	for i in range(tim):
		var even = i % 2 == 0
		shot.tween_interval(MOVE_TIME / 2)
		for j in range(4):
			var start_pos = (
				(swing_even_position + vector / 5 * (j + 1))
				if even
				else (swing_even_position + vector / 5 * (4 - j))
			)
			shot.tween_callback(
				(
					swing_shot_beam.bind(start_pos, to)
					if even
					else swing_shot_beam.bind(start_pos, 2 * start_pos - (point * 2 - to))
				)
			)
		shot.tween_interval(MOVE_TIME / 2 + WAIT_TIME)


#--------------

# ---motionless---
# 原地旋转攻击, 并释放星型弹幕


# to: 攻击时所在位置
# time: 攻击持续时间
# 先旋转到to, 再攻击time
func motionless_shot_beam(num: int):
	var beams: Array[Node] = []
	for i in range(num):
		beams.push_back(enchanted_beam.instantiate())
		beams[i].direction = rad_to_vector(rotation + TAU * i / num + randf_range(-PI / 6, PI / 6))
		beams[i].position = position + beams[i].direction * 10
		beams[i].speed = 800
	for i in beams:
		get_parent().add_child(i)


func motionless_attack(to: Vector2, time: float):
	const MOVE_TIME = 0.7
	const SHOT_TIME = 0.2
	create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC).tween_property(
		self, "position", to, MOVE_TIME
	)
	var sword_rotate = create_tween()
	sword_rotate.tween_interval(MOVE_TIME)
	sword_rotate.tween_property(self, "rotation", rotation + 3 * TAU * time, time)
	sword_rotate.tween_callback(exit)

	var shot = create_tween()
	shot.tween_interval(MOVE_TIME)
	var times = floor(time / SHOT_TIME)
	for i in range(times):
		shot.tween_callback(motionless_shot_beam.bind(6))
		shot.tween_interval(SHOT_TIME)


#---rotate---
# 旋转攻击, 并释放星型弹幕

# point, to: 首尾点


func rotate_shot_beam(num: int):
	var beams: Array[Node] = []
	for i in range(num):
		beams.push_back(enchanted_beam.instantiate())
		beams[i].direction = rad_to_vector(rotation + TAU * i / num)
		beams[i].position = position + beams[i].direction * 10
		# 不让他自己动, 通过下列动画完成
		beams[i].speed = 0

	# 平均速度
	const V = 800
	var move_time = 2000 / V
	for beam in beams:
		get_parent().add_child(beam)
		var tween = create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_CUBIC)
		tween.tween_property(beam, "position", beam.position + 2000 * beam.direction, move_time)


func rotate_attack(to: Vector2, tim: int):
	const SHOT_TIME = 0.4
	const WAIT_TIME = 0.1
	# 移动
	var move_time = (to - position).length() / 600
	var move = create_tween()
	move.tween_property(self, "position", to, move_time)
	move.tween_interval(0.1)
	move.tween_callback(exit)

	# 旋转
	var end_rotation = rotation + TAU * 3.5 * move_time
	create_tween().tween_property(self, "rotation", end_rotation, move_time + WAIT_TIME)

	var shot = create_tween()
	shot.tween_callback(rotate_shot_beam.bind(tim))
	# 攻击次数
	var times = floor(move_time / SHOT_TIME)
	for i in range(times):
		shot.tween_interval(SHOT_TIME)
		shot.tween_callback(rotate_shot_beam.bind(tim))


#---vertical---
# 垂直攻击
# 在这两个点间反复横跳
# 垂直攻击发射弹幕
# 弹幕与剑头方向一致


func vertical_shot_beam(pos: Vector2, rad: float):
	var beam = enchanted_beam.instantiate()
	beam.direction = rad_to_vector(rad)
	beam.position = pos + beam.direction.normalized() * 30
	beam.speed = 650
	get_parent().add_child(beam)


# point, to: 首尾点
# tim: 重复次数
# 在point 和 to 间反复横跳
# 剑头方向为 point 指向 to 的向量转 PI / 2
func vertical_attack(point: Vector2, to: Vector2, tim: int):
	const MOVE_TIME = 0.3
	const WAIT_TIME = 0.3
	var shot_rad = point.angle_to_point(to) + PI / 2
	var attack = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUAD)
	attack.tween_property(self, "position", point, 1)
	attack.parallel().tween_property(self, "rotation", shot_rad + PI / 4, 1)

	for i in range(tim):
		var even = is_even(i)
		attack.tween_property(self, "position", to if even else point, MOVE_TIME)
		attack.tween_interval(WAIT_TIME)
	attack.tween_callback(exit)

	var shot = create_tween()
	shot.tween_interval(1)

	for i in range(tim):
		var even = is_even(i)
		var unit = 70 * (to - point if even else point - to).normalized()
		var get_pos = func(x): return (point if even else to) + unit * x
		var num = floorf((point - to).length() / unit.length()) + 1
		var unit_time = MOVE_TIME / num
		for j in range(num):
			shot.tween_callback(vertical_shot_beam.bind(get_pos.call(j), shot_rad))
			shot.tween_interval(unit_time)
		shot.tween_interval(WAIT_TIME)


#---rush---
# 冲刺攻击
# point, to: 首尾点
# 剑头方向为 point 指向 to
func rush_attack(point: Vector2, to: Vector2):
	const PRE_MOVE_TIME = 0.8
	const MOVE_TIME = 0.6
	const ADD_TIME = -0.25
	var move = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC)
	move.tween_property(self, "position", point, PRE_MOVE_TIME)
	var end_rotation = point.angle_to_point(to) + PI / 4
	move.parallel().tween_property(self, "rotation", end_rotation, PRE_MOVE_TIME)
	move.tween_property(self, "position", to, MOVE_TIME)
	move.tween_callback(exit)

	var unit = 70 * (to - point).normalized()
	var get_pos = func(i): return point + unit * i
	var num = floorf((point - to).length() / unit.length()) + 1
	var unit_time = MOVE_TIME / (2 * num)
	var shot = create_tween()
	shot.tween_interval(PRE_MOVE_TIME)
	var get_time = func(i): return MOVE_TIME + ADD_TIME - unit_time * (i - 1)

	for i in range(num):
		shot.tween_callback(
			rush_shot_beam.bind(get_pos.call(i), point.angle_to_point(to), get_time.call(i))
		)
		shot.tween_interval(unit_time)


func rush_shot_beam(pos: Vector2, rad: float, wait_time: float):
	const V = 800
	var beam = enchanted_beam.instantiate()
	beam.direction = rad_to_vector(rad)
	beam.position = pos + beam.direction.normalized() * 30
	beam.speed = 0
	get_parent().add_child(beam)
	var move_time = 2000 / V
	var tween = create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_CUBIC)
	tween.tween_interval(wait_time)
	tween.tween_property(beam, "position", beam.position + 2000 * beam.direction, move_time)
