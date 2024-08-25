extends RayCast2D

@export var cast_speed = 7000.0
@export var max_length = 1400.0
@export var growth_time = 0.5

# 标记是否发送
var is_casting = false
var tween
# 该线是两个点的，用于标记激光
@onready var fill = $Line2D
@onready var casting_particles = $CastingParticles
# 碰撞后产生的粒子效果
@onready var collision_particles = $CollisionParticles
# 激光产生的粒子
@onready var beam_particles = $BeamParticles
# 激光的宽度
@onready var line_width = fill.width
# 激光的方向
var direct: Vector2 = Vector2.RIGHT


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	# 无激光绘制时关闭process减少性能开销
	set_physics_process(false)
	fill.points[1] = Vector2.ZERO
	tween = create_tween()


func _physics_process(delta: float) -> void:
	target_position = (target_position + self.direct * cast_speed * delta).limit_length(max_length)
	cast_beam()


# 初始化
func init(pos: Vector2):
	self.global_position = pos


# 设置激光的方向
func set_direct(direct_arg: Vector2) -> void:
	self.direct = direct_arg.normalized()
	var rad = direct.y / direct.x
	# 激光的角度
	beam_particles.rotation = rad


func cast_beam():
	# 当前的终点
	var cast_point = target_position
	# 更新射线
	force_raycast_update()
	# 检测是否碰撞
	var status = is_colliding()
	# 如果碰撞，产生碰撞粒子
	collision_particles.emitting = status
	if status:
		cast_point = to_local(get_collision_point())
		# 让碰撞粒子旋转某个角度
		collision_particles.global_rotation = get_collision_normal().angle()
		# 让碰撞粒子移动到碰撞点
		collision_particles.global_position = get_collision_point()
	fill.points[1] = cast_point
	beam_particles.position = cast_point * 0.5
	beam_particles.process_material.emission_box_extents.x = cast_point.length() * 0.5


# 设置激光开启关闭
func set_is_casting(cast: bool) -> void:
	is_casting = cast
	if is_casting:
		target_position = Vector2.ZERO
		fill.points[1] = target_position
		appear()
	else:
		fill.points[1] = Vector2.ZERO
		collision_particles.emitting = false
		disappear()
	set_physics_process(is_casting)
	beam_particles.emitting = is_casting
	casting_particles.emitting = is_casting


# 光柱渐渐出现
func appear():
	tween.kill()
	tween = create_tween()
	tween.tween_property(fill, "width", line_width, growth_time * 2).from(0)


# 光柱消失
func disappear():
	tween.kill()
	tween = create_tween()
	tween.tween_property(fill, "width", 0, growth_time * 1.5).from(line_width)
