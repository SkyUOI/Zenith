extends RayCast2D

@export var cast_speed = 7000.0
@export var max_length = 1400.0
@export var growth_time = 0.5

var is_casting = false
var tween
@onready var fill = $Line2D
@onready var casting_particles = $CastingParticles
@onready var collision_particles = $CollisionParticles
@onready var beam_particles = $BeamParticles
@onready var line_width = fill.width


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_physics_process(false)
	fill.points[1] = Vector2.ZERO
	tween = create_tween()


func _physics_process(delta: float) -> void:
	target_position = (target_position + Vector2.RIGHT * cast_speed * delta).limit_length(
		max_length
	)
	cast_beam()


func cast_beam():
	var cast_point = target_position
	force_raycast_update()
	var status = is_colliding()
	collision_particles.emitting = status
	if status:
		cast_point = to_local(get_collision_point())
		collision_particles.global_rotation = get_collision_normal().angle()
		collision_particles.global_position = get_collision_point()
	fill.points[1] = cast_point
	beam_particles.position = cast_point * 0.5
	beam_particles.process_material.emission_box_extents.x = cast_point.length() * 0.5


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


func appear():
	tween.kill()
	tween = create_tween()
	tween.tween_property(fill, "width", line_width, growth_time * 2).from(0)


func disappear():
	tween.kill()
	tween = create_tween()
	tween.tween_property(fill, "width", 0, growth_time).from(line_width)
