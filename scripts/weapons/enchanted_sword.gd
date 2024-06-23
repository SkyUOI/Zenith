extends EnchantedSword

var finished: bool

# 操作序列(也许只是测试用)
var opers: Array[Callable]
signal attack_finished



func _ready():
	
	opers = [
		func(): swingAttack(Vector2(330, 330), Vector2(800, 300), 4),
		func(): swingAttack(Vector2(430, 200), Vector2(800, 750), 4),
		func(): swingAttack(Vector2(830, 330), Vector2(300, 400), 4),
		func(): verticalAttack(Vector2(200, 100), Vector2(900, 100), 4),
		func(): verticalAttack(Vector2(200, 500), Vector2(600, 100), 4),
		#func(): rotateAttack(Vector2(1000, 100), 6),
		#func(): rotateAttack(Vector2(500, 500), 6),
		#func(): motionlessAttack(Vector2(500, 300), 5),
		#func(): motionlessAttack(Vector2(600, 400), 6),
		#func(): rushAttack(Vector2(130, 230), Vector2(800, 400)),
		#func(): rushAttack(Vector2(800, 600), Vector2(200, 200)),
	]
	if get_tree().current_scene == self:
		exit()


# fight调用这个
# 开始
func fightStart():
	exit()


@export var enchanted_beam: PackedScene
# 当前动作对应函数
var now_func: Callable


func start() -> bool:
	# todo
	if !finished:
		return false
	finished = false
	#now_func = func_name
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


func is_even(x: int) -> bool:
	return x % 2 == 0

# 角度对应向量
func radToVector(rad: float) -> Vector2:
	return Vector2(cos(rad), sin(rad))


## 旋转不超过PI到指定角度并移动到指定位置
## end_rotation:剑头的角度
## Return:是否达到指定位置
#func normalMove(end_point: Vector2, end_rotation: float, speed: float, delta: float) -> bool:
	#var direction = radToVector(rotation).angle_to(radToVector(swordToRotation(end_rotation)))
	#var vector = end_point - position
	#if vector.length() == 0:
		#return true
	#var rotate_speed = direction / vector.length() * speed
	#return rotateMove(end_point, speed, rotate_speed, delta)


# 旋转几周并移动到指定位置(不考虑剑头方向)
## return: 是否达到指定位置
#func rotateMove(end_point: Vector2, speed: float, rotate_speed: float, delta: float) -> bool:
	#rotation += rotate_speed * delta
	## 避免到达后振动
	#if (position - end_point).length() <= speed / 100:
		#position = end_point
		#return true
	## 起点指向终点的向量
	#var vector = (end_point - position).normalized() * speed
	#position += vector * delta
	#rotation += rotate_speed * delta
	#return false


# ---swing---
var start_point: Vector2
var target: Vector2
var times: int
var totTimes: int
var toTargetRad: float
# 挥舞后的等待
var wait: Timer
var shot: Timer


# 发射附魔光束
func swingShotBeam(start_pos : Vector2, end_pos : Vector2):
	var beam = enchanted_beam.instantiate()	
	beam.direction = end_pos - start_pos 
	beam.position = start_pos 
	beam.speed = 750
	get_parent().add_child(beam)

# 启动挥舞动作
# point: 挥舞路径中点
# to: 目标点
func swingAttack(point: Vector2, to: Vector2, tim: int):
	if !start():
		return
	var to_target_rad = point.angle_to_point(to)
	var swing_odd_rad = to_target_rad + (-PI * 2 / 3)
	var swing_even_rad = to_target_rad + (PI * 1.5 / 3)
	var swing_odd_position = point + 170 * radToVector(to_target_rad + (-PI / 2))
	var swing_even_position = point + 170 * radToVector(to_target_rad + PI / 2)
	var swing = create_tween().set_ease(Tween.EASE_OUT)\
			.set_trans(Tween.TRANS_CUBIC)
	swing.tween_property(self, "position", point, 1)
	swing.parallel().tween_property(self, "rotation", to_target_rad, 1)
	for i in range(tim):
		var even = is_even(i)
		swing.tween_property(self, "position", swing_even_position \
				if even else swing_odd_position, 0.4)
		swing.parallel().tween_property(self, "rotation", swing_even_rad\
				if even else swing_odd_rad, 0.4)
	swing.tween_callback(exit)
	
	var vector = swing_odd_position - swing_even_position
	var getPos = func(i : int) -> Vector2:
		return (swing_even_position + vector / 5 * (i + 1))
	var shot = create_tween()
	shot.tween_interval(1)
	for i in range(tim):
		var even = (i % 2 == 0)
		shot.tween_interval(0.2)
		for j in range(4):
			var start_pos = getPos.call(j) if even else getPos.call(3 - j)
			shot.tween_callback(swingShotBeam.bind(start_pos, to)\
					if even else swingShotBeam.bind(start_pos, 2 * start_pos - (point * 2 - to)))
		shot.tween_interval(0.2)
	

#--------------
#
##---motionless---
## 原地旋转攻击, 并释放星型弹幕
#
#var last_rotation = 0.0
#
#
#func motionlessShotBeam():
	#var beams: Array[Node] = []
	##让整体偏移
	#last_rotation += randf_range(-PI / 10, PI / 10)
	#for i in range(totTimes):
		#beams.push_back(enchanted_beam.instantiate())
		#beams[i].get_node("Start").wait_time = 0.05
		#beams[i].direction = radToVector(
			#last_rotation + 2 * PI * i / totTimes + randf_range(-PI / 20, PI / 20)
		#)
		#beams[i].position = position
		#beams[i].speed = 800
	#for beam in beams:
		#get_parent().add_child(beam)
		#beam.get_node("Start").start()
#
#
#func motionlessExit():
	#times = 0
	#wait.queue_free()
	#shot.queue_free()
	#exit()
#
#
## to: 攻击时所在位置
## time: 攻击持续时间
## 先旋转到to, 再攻击time
#func motionlessStart(to: Vector2, time: float):
	#if !start(motionless):
		#return
	#target = to
	#totTimes = 6
	## 初始化wait
	#wait = Timer.new()
	#wait.one_shot = true
	#wait.wait_time = time
	#add_child(wait)
	## 初始化shot
	#shot = Timer.new()
	#shot.wait_time = 0.15
	#add_child(shot)
	#shot.timeout.connect(motionlessShotBeam)
#
#
#func motionless(delta):
	#if wait.time_left == 0 && times > 0:
		#motionlessExit()
		#return
	#if shot.time_left == 0 && times > 0:
		#shot.start()
#
	## 旋转移向target
	#var arrive: bool = false
	#if times == 0:
		#arrive = rotateMove(target, 1000, 4 * PI, delta)
		#if arrive:
			#wait.start()
			#times += 1
	#else:
		#rotation += PI * 6 * delta
#
#
##------------
#
##---rotate---
## 旋转攻击, 并释放星型弹幕
#
## point, to: 首尾点
#
#
#func rotateShotBeam():
	#var beams: Array[Node] = []
	#for i in range(totTimes):
		#beams.push_back(enchanted_beam.instantiate())
		#beams[i].get_node("Start").wait_time = 0.35
		#beams[i].direction = radToVector(rotation + 2 * PI * i / totTimes)
		#beams[i].position = position
		#beams[i].speed = 800
	#for beam in beams:
		#get_parent().add_child(beam)
		#beam.get_node("Start").start()
#
#
#func rotateExit():
	#times = 0
	#wait.queue_free()
	#shot.queue_free()
	#exit()
#
#
#func rotateStart(to: Vector2, tim: int):
	#if !start(rotateAttack):
		#return
	#target = to
	## 一次几个弹幕
	#totTimes = tim
	## 初始化wait
	#wait = Timer.new()
	#wait.one_shot = true
	#wait.wait_time = 0.15
	#add_child(wait)
	## 初始化shot
	#shot = Timer.new()
	#shot.wait_time = 0.2
	#add_child(shot)
	#shot.timeout.connect(rotateShotBeam)
#
#
#func rotateAttack(delta):
	## 到达且wait超时
	#if wait.time_left == 0 && times > 0:
		#verticalExit()
		#return
	#if shot.time_left == 0:
		#shot.start()
#
	## 旋转移向target
	#var arrive = rotateMove(target, 1000, 4 * PI, delta)
	#if arrive && wait.time_left == 0:
		#wait.start()
		#shot.stop()
		#times += 1
#
#
#------------

#---vertical---
# 垂直攻击
# 在这两个点间反复横跳

# 垂直攻击发射弹幕
# 弹幕与剑头方向一致
func verticalShotBeam(pos : Vector2, rad : float):
	var beam = enchanted_beam.instantiate()
	beam.direction = radToVector(rad)
	beam.position = pos + beam.direction.normalized() * 30
	beam.speed = 650
	get_parent().add_child(beam)

# point, to: 首尾点
# tim: 重复次数
# 在point 和 to 间反复横跳
# 剑头方向为 point 指向 to 的向量转 PI / 2
func verticalAttack(point: Vector2, to: Vector2, tim: int):
	const move_time = 0.45
	const wait_time = 0.3
	if !start():
		return
	var shot_rad = point.angle_to_point(to) + PI / 2
	var attack = create_tween().set_ease(Tween.EASE_OUT)\
			.set_trans(Tween.TRANS_QUAD)
	attack.tween_property(self, "position", point, 1)
	attack.parallel().tween_property(self, "rotation", shot_rad + PI / 4, 1)
	
	for i in range(tim):
		var even = is_even(i)
		attack.tween_property(self, "position", to if even else point, move_time)	
		attack.tween_interval(wait_time)
	attack.tween_callback(exit)
	
	var shot = create_tween()
	shot.tween_interval(1)
	
	for i in range(tim):
		var even = is_even(i)
		var unit = 70 * (to - point if even else point - to).normalized()
		var getPos = func(i):
			return (point if even else to) + unit * i
		var num = floorf((point - to).length() / unit.length()) + 1
		var unit_time = move_time / num
		for j in range(num):
			shot.tween_callback(verticalShotBeam\
					.bind(getPos.call(j), shot_rad))
			shot.tween_interval(unit_time)
		shot.tween_interval(wait_time)
#
#

#
#
#func verticalExit():
	#times = 0
	#wait.queue_free()
	#shot.queue_free()
	#exit()
#
#
#func vertical(delta: float):
	#if wait.time_left != 0:
		#return
	#if shot.time_left == 0 && times != 0:
		#shot.start()
	#if times >= totTimes:
		#verticalExit()
		#return
#
	##在两个点之间反复横跳
	#var arrive = normalMove(
		#start_point if evenTimes() else target, (target - start_point).angle() + PI / 2, 1600, delta
	#)
	#if arrive:
		#wait.start()
		#shot.stop()
		#times += 1
#
#
##-----------
#
##-----------
#
#
##---rush---
## 冲刺攻击
## point, to: 首尾点
## 剑头方向为 point 指向 to
#func rushStart(point: Vector2, to: Vector2):
	#if !start(rush):
		#return
	#start_point = point
	#totTimes = 2
	#target = to
	## 初始化wait
	#wait = Timer.new()
	#wait.one_shot = true
	#wait.wait_time = 0.06
	#add_child(wait)
	## 初始化shot
	#shot = Timer.new()
	#shot.wait_time = 0.04
	#add_child(shot)
	#shot.timeout.connect(rushShotBeam)
#
#
## 弹幕与剑头方向一致
#func rushShotBeam():
	#var beam = enchanted_beam.instantiate()
	#beam.get_node("Start").wait_time = 0.1
	#beam.direction = radToVector((target - start_point).angle())
	#beam.position = position - beam.direction.normalized() * 50
	#beam.speed = 1000
	#get_parent().add_child(beam)
	#beam.set_process(0)
	#attack_finished.connect(beam.start)
#
#
#func rushExit():
	#times = 0
	#wait.queue_free()
	#shot.queue_free()
	#exit()
#
#
#func rush(delta: float):
	#if wait.time_left != 0:
		#return
	#if shot.time_left == 0 && times != 0:
		#shot.start()
	#if times >= totTimes:
		#rushExit()
		#return
#
	#var arrive = normalMove(
		#start_point if evenTimes() else target, (target - start_point).angle(), 2000, delta
	#)
	#if arrive:
		#wait.start()
		#shot.stop()
		#times += 1
#
#
##-----------
#
#
