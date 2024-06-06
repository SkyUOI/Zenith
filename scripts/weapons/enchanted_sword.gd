extends Sprite2D

var finished: bool


func _ready():
	finished = true
	swingStart(Vector2(330, 330), Vector2(600, 300), 4)


@export var enchanted_beam: PackedScene
var now_func: Callable  # 当前动作对应函数


func start(func_name: Callable) -> bool:
	if !finished:  # 没完成
		return false
	finished = false
	now_func = func_name
	return true


func exit():
	finished = true


func radToVector(rad: float) -> Vector2:
	return Vector2(cos(rad), sin(rad))


# 剑头指向某一方向的rotation
func swordToRotation(rad: float) -> float:
	return rad + PI / 4


#返回是否达到指定位置
# 旋转并移动到指定位置                     #剑头的角度
func normalMove(end_point: Vector2, end_rotation: float, speed: float, delta: float) -> bool:
	if (position - end_point).length() <= speed / 100:
		return true
	# 起点指向终点的向量
	var vector = end_point - position
	# 起点到终点转的角度
	var direction = radToVector(rotation).angle_to(radToVector(swordToRotation(end_rotation)))
	var rotate_speed = direction / vector.length() * speed
	vector = vector.normalized() * speed

	rotation += rotate_speed * delta  #旋转
	position += vector * delta  # 向指定位置移动
	return false


# ---swing---
var start_point: Vector2
var target: Vector2
var times: int
var totTimes: int
var toTargetRad: float
var wait: Timer  # 挥舞后的等待
var shot: Timer


func swingExit():
	exit()
	wait.queue_free()
	shot.queue_free()


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
	shot.wait_time = 0.08
	add_child(shot)
	shot.timeout.connect(swingShotBeam)
	toTargetRad = start_point.angle_to_point(target)


func evenTimes() -> bool:
	return times % 2 == 0


#发射附魔光束
func swingShotBeam():
	var beam = enchanted_beam.instantiate()
	beam.get_node("Start").wait_time = 0.05
	#偶数次弹幕收拢
	beam.direction = target - position
	if !evenTimes():  #奇数次弹幕发散
		beam.direction = Vector2(beam.direction.x, -beam.direction.y)
	beam.position = position + beam.direction.normalized() * 50
	beam.speed = 1000
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
	var to_position = 150 * radToVector(toTargetRad + (-PI / 2 if evenTimes() else PI / 2))
	var arrive = normalMove(
		start_point + to_position,
		toTargetRad + ((-PI * 1.8 / 3) if evenTimes() else (PI / 3)),
		1150,
		delta
	)
	if arrive:
		wait.start()
		shot.stop()
		times += 1


# -----------

#---vertical---
# 垂直攻击

# 在这两个点间反复横跳


func verticalStart(point: Vector2, to: Vector2):
	if !start(vertical):
		return
	start_point = point
	target = to
	# 初始化wait
	wait = Timer.new()
	wait.one_shot = true
	wait.wait_time = 0.35
	add_child(wait)
	# 初始化shot
	shot = Timer.new()
	shot.wait_time = 0.08
	add_child(shot)
	shot.timeout.connect(verticalShotBeam)


func verticalShotBeam():
	var beam = enchanted_beam.instantiate()
	beam.get_node("Start").wait_time = 0.05
	#偶数次弹幕收拢
	beam.direction = target - position
	if !evenTimes():  #奇数次弹幕发散
		beam.direction = Vector2(beam.direction.x, -beam.direction.y)
	beam.position = position + beam.direction.normalized() * 50
	beam.speed = 1000
	get_parent().add_child(beam)
	beam.get_node("Start").start()


func vertical(delta: float):
	if wait.time_left != 0:
		return
	if shot.time_left == 0 && times != 0:
		shot.start()
	if times >= 4:
		swingExit()
		return
	# 移到(从start_point向(start_point与target连线垂直方向)一定距离)
	var to_position = 150 * radToVector(toTargetRad + (-PI / 2 if evenTimes() else PI / 2))
	var arrive = normalMove(
		start_point + to_position,
		toTargetRad + ((-PI * 1.8 / 3) if evenTimes() else (PI / 3)),
		1150,
		delta
	)
	if arrive:
		wait.start()
		shot.stop()
		times += 1


#--------------
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !finished && now_func:
		now_func.call(delta)
	# else :
	# 应该有个默认状态保持竖直朝下 并上下漂浮?
