extends Sprite2D


var finished : bool
func _ready():
	finished = true
	swingStart(Vector2(330, 330), Vector2(600, 300))


var now_func : Callable # 当前动作对应函数


func start(func_name : Callable):
	if !finished: # 没完成 
		return
	finished = false
	now_func = func_name

func exit():
	finished = true	
	
func radToVector(rad : float) -> Vector2:
	return Vector2(cos(rad), sin(rad))
# 剑头指向某一方向的rotation
func swordToRotation(rad : float) -> float:
	return rad + PI / 4
	
#返回是否达到指定位置
# 旋转并移动到指定位置                     #剑头的角度 
func normalMove(end_point : Vector2, end_rotation : float , speed : float, delta : float) -> bool: 
	if (position - end_point).length() <= speed / 100:
		return true
	# 起点指向终点的向量
	var vector = end_point - position
	# 起点到终点转的角度
	var direction = radToVector(rotation).angle_to(radToVector(swordToRotation(end_rotation)))
	var rotate_speed = direction / vector.length() * speed
	vector = vector.normalized() * speed
	
	rotation += rotate_speed * delta #旋转
	position += vector * delta# 向指定位置移动	
	return false
	
# ---swing---	
var start_point : Vector2
var target : Vector2
var times : int
func swingStart(point : Vector2, to : Vector2):
	start(swing)
	start_point = point
	target = to

		
	
# 在大概start_point这个位置, 向target挥舞攻击	
func swing(delta : float):
	if times >= 5:
		exit()
		return
	# 移到(从start_point向(start_point与target连线垂直方向)一定距离)
	var even = times % 2 == 0
	var rad = start_point.angle_to_point(target)
	var to_position = 200 * radToVector(rad + (- PI / 2 if even else PI / 2))
	var arrive = normalMove(start_point + to_position, rad -PI / 3 if even else PI / 3, 600, delta) 
	if arrive: 
		times += 1
	
# -----------	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !finished && now_func:
		now_func.call(delta)
	# else : 
		# 应该有个默认状态保持竖直朝下 并上下漂浮?
