extends Sprite2D

var finished: bool

# 操作序列(也许只是测试用)
var opers: Array[Callable]
signal attack_finished


func _ready():
	if get_tree().current_scene != self:
		set_process(0)
	opers = [
		func(): swingStart(Vector2(330, 330), Vector2(800, 300), 4),
		func(): swingStart(Vector2(430, 200), Vector2(800, 750), 4),
		func(): swingStart(Vector2(830, 330), Vector2(300, 400), 4),
		func(): verticalStart(Vector2(200, 100), Vector2(900, 100), 4),
		func(): verticalStart(Vector2(200, 500), Vector2(600, 100), 4),
		func(): rotateStart(Vector2(1000, 100), 6),
		func(): rotateStart(Vector2(500, 500), 6),
		func(): motionlessStart(Vector2(500, 300), 5),
		func(): motionlessStart(Vector2(600, 400), 6),
		func(): rushStart(Vector2(130, 230), Vector2(800, 400)),
		func(): rushStart(Vector2(800, 600), Vector2(200, 200)),
	]
	exit()


# fight调用这个
# 开始
func fightStart():
	set_process(1)


@export var enchanted_beam: PackedScene
# 当前动作对应函数
var now_func: Callable


func start(func_name: Callable) -> bool:
	# todo
	if !finished:
		return false
	finished = false
	now_func = func_name
	return true


# 当前是第几个oper(用于opers)
var oper_num: int = -1


# 完成某一动作
func exit():
	finished = true
	attack_finished.emit()
	# 也许只是测试用, 前往下一个操作
	nextOper()


# 前往下一个动作
func nextOper():
	oper_num += 1
	if oper_num >= opers.size():
		attack_finished.emit()
		return
	opers[oper_num].call()


# 角度对应向量
func radToVector(rad: float) -> Vector2:
	return Vector2(cos(rad), sin(rad))


# 剑头指向某一方向的rotation
func swordToRotation(rad: float) -> float:
	return rad + PI / 4


# 旋转不超过PI到指定角度并移动到指定位置
# end_rotation:剑头的角度
# Return:是否达到指定位置
func normalMove(end_point: Vector2, end_rotation: float, speed: float, delta: float) -> bool:
	var direction = radToVector(rotation).angle_to(radToVector(swordToRotation(end_rotation)))
	var vector = end_point - position
	if vector.length() == 0:
		return true
	var rotate_speed = direction / vector.length() * speed
	return rotateMove(end_point, speed, rotate_speed, delta)


# 旋转几周并移动到指定位置(不考虑剑头方向)
# return: 是否达到指定位置
func rotateMove(end_point: Vector2, speed: float, rotate_speed: float, delta: float) -> bool:
	rotation += rotate_speed * delta
	# 避免到达后振动
	if (position - end_point).length() <= speed / 100:
		position = end_point
		return true
	# 起点指向终点的向量
	var vector = (end_point - position).normalized() * speed
	position += vector * delta
	rotation += rotate_speed * delta
	return false


# ---swing---
var start_point: Vector2
var target: Vector2
var times: int
var totTimes: int
var toTargetRad: float
# 挥舞后的等待
var wait: Timer
var shot: Timer


func swingExit():
	times = 0
	wait.queue_free()
	shot.queue_free()
	exit()


# 启动挥舞动作
# point: 挥舞路径中点
# to: 目标点
func swingStart(point: Vector2, to: Vector2, tim: int):
	if !start(swing):
		return
	totTimes = tim
	start_point = point
	target = to
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = 0.35
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.06
	add_child(shot)
	shot.timeout.connect(swingShotBeam)
	toTargetRad = start_point.angle_to_point(target)


# return: 次数是否为偶数
func evenTimes() -> bool:
	return times % 2 == 0


# 发射附魔光束
func swingShotBeam():
	var beam = enchanted_beam.instantiate()
	beam.get_node("Start").wait_time = 0.05
	var vector = target - position

	#偶数次弹幕收拢
	if evenTimes():
		beam.direction = vector
	#奇数次弹幕发散
	else:
		var basic_vector = target - start_point
		var rad = basic_vector.angle_to(vector)
		beam.direction = radToVector(basic_vector.angle() - rad)

	beam.position = position + beam.direction.normalized() * 50
	beam.speed = 800
	get_parent().add_child(beam)
	beam.get_node("Start").start()


# 在大概start_point这个位置, 向target挥舞攻击
func swing(delta: float):
	if wait.time_left != 0:
		return
	if shot.time_left == 0 && times != 0:
		shot.start()
	if times >= totTimes:
		swingExit()
		return
	# 移到(从start_point向(start_point与target连线垂直方向)一定距离)
	var to_position = 170 * radToVector(toTargetRad + (-PI / 2 if evenTimes() else PI / 2))
	var arrive = normalMove(
		start_point + to_position,
		toTargetRad + ((-PI * 1.8 / 3) if evenTimes() else (PI / 3)),
		1700,
		delta
	)
	if arrive:
		wait.start()
		shot.stop()
		times += 1


# -----------

#--------------

#---motionless---
# 原地旋转攻击, 并释放星型弹幕

var last_rotation = 0.0


func motionlessShotBeam():
	var beams: Array[Node] = []
	#让整体偏移
	last_rotation += randf_range(-PI / 10, PI / 10)
	for i in range(totTimes):
		beams.push_back(enchanted_beam.instantiate())
		beams[i].get_node("Start").wait_time = 0.05
		beams[i].direction = radToVector(
			last_rotation + 2 * PI * i / totTimes + randf_range(-PI / 20, PI / 20)
		)
		beams[i].position = position
		beams[i].speed = 800
	for beam in beams:
		get_parent().add_child(beam)
		beam.get_node("Start").start()


func motionlessExit():
	times = 0
	wait.queue_free()
	shot.queue_free()
	exit()


# to: 攻击时所在位置
# time: 攻击持续时间
# 先旋转到to, 再攻击time
func motionlessStart(to: Vector2, time: float):
	if !start(motionless):
		return
	target = to
	totTimes = 6
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = time
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.15
	add_child(shot)
	shot.timeout.connect(motionlessShotBeam)


func motionless(delta):
	if wait.time_left == 0 && times > 0:
		motionlessExit()
		return
	if shot.time_left == 0 && times > 0:
		shot.start()

	# 旋转移向target
	var arrive: bool = false
	if times == 0:
		arrive = rotateMove(target, 1000, 4 * PI, delta)
		if arrive:
			wait.start()
			times += 1
	else:
		rotation += PI * 6 * delta


#------------

#---rotate---
# 旋转攻击, 并释放星型弹幕

# point, to: 首尾点


func rotateShotBeam():
	var beams: Array[Node] = []
	for i in range(totTimes):
		beams.push_back(enchanted_beam.instantiate())
		beams[i].get_node("Start").wait_time = 0.35
		beams[i].direction = radToVector(rotation + 2 * PI * i / totTimes)
		beams[i].position = position
		beams[i].speed = 800
	for beam in beams:
		get_parent().add_child(beam)
		beam.get_node("Start").start()


func rotateExit():
	times = 0
	wait.queue_free()
	shot.queue_free()
	exit()


func rotateStart(to: Vector2, tim: int):
	if !start(rotateAttack):
		return
	target = to
	# 一次几个弹幕
	totTimes = tim
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = 0.15
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.2
	add_child(shot)
	shot.timeout.connect(rotateShotBeam)


func rotateAttack(delta):
	# 到达且wait超时
	if wait.time_left == 0 && times > 0:
		verticalExit()
		return
	if shot.time_left == 0:
		shot.start()

	# 旋转移向target
	var arrive = rotateMove(target, 1000, 4 * PI, delta)
	if arrive && wait.time_left == 0:
		wait.start()
		shot.stop()
		times += 1


#------------

#---vertical---
# 垂直攻击
# 在这两个点间反复横跳


# point, to: 首尾点
# tim: 重复次数
# 在point 和 to 间反复横跳
# 剑头方向为 point 指向 to 的向量转 PI / 2
func verticalStart(point: Vector2, to: Vector2, tim: int):
	if !start(vertical):
		return
	start_point = point
	totTimes = tim
	target = to
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = 0.35
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.05
	add_child(shot)
	shot.timeout.connect(verticalShotBeam)


# 垂直攻击发射弹幕
# 弹幕与剑头方向一致
func verticalShotBeam():
	var beam = enchanted_beam.instantiate()
	beam.get_node("Start").wait_time = 0.05
	beam.direction = radToVector((target - start_point).angle() + PI / 2)
	beam.position = position + beam.direction.normalized() * 50
	beam.speed = 800
	get_parent().add_child(beam)
	beam.get_node("Start").start()


func verticalExit():
	times = 0
	wait.queue_free()
	shot.queue_free()
	exit()


func vertical(delta: float):
	if wait.time_left != 0:
		return
	if shot.time_left == 0 && times != 0:
		shot.start()
	if times >= totTimes:
		verticalExit()
		return

	#在两个点之间反复横跳
	var arrive = normalMove(
		start_point if evenTimes() else target, (target - start_point).angle() + PI / 2, 1600, delta
	)
	if arrive:
		wait.start()
		shot.stop()
		times += 1


#-----------

#-----------


#---rush---
# 冲刺攻击
# point, to: 首尾点
# 剑头方向为 point 指向 to
func rushStart(point: Vector2, to: Vector2):
	if !start(rush):
		return
	start_point = point
	totTimes = 2
	target = to
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = 0.06
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.04
	add_child(shot)
	shot.timeout.connect(rushShotBeam)


# 弹幕与剑头方向一致
func rushShotBeam():
	var beam = enchanted_beam.instantiate()
	beam.get_node("Start").wait_time = 0.1
	beam.direction = radToVector((target - start_point).angle())
	beam.position = position - beam.direction.normalized() * 50
	beam.speed = 1000
	get_parent().add_child(beam)
	beam.set_process(0)
	attack_finished.connect(beam.start)


func rushExit():
	times = 0
	wait.queue_free()
	shot.queue_free()
	exit()


func rush(delta: float):
	if wait.time_left != 0:
		return
	if shot.time_left == 0 && times != 0:
		shot.start()
	if times >= totTimes:
		rushExit()
		return

	var arrive = normalMove(
		start_point if evenTimes() else target, (target - start_point).angle(), 2000, delta
	)
	if arrive:
		wait.start()
		shot.stop()
		times += 1


#-----------


func _process(delta):
	if !finished && now_func:
		now_func.call(delta)
	# else :
	# 应该有个默认状态保持竖直朝下 并上下漂浮?
