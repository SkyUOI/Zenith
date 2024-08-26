extends Node

@export var terra_blade_beam : PackedScene
var tween = create_tween()

# Called when the node enters the scene tree for the first time.
func _ready():
	begin()
	
	tween.tween_callback(func():
		await get_tree().create_timer(0.2).timeout
		line_call.call(Vector2(200, 200), Vector2(700,450), 10, shot_falling_beam)
	)
	tween.tween_property($Sprite2D, "position", Vector2(700, 450), 0.5).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC)
	tween.tween_interval(10)

func line_call(from, to, num, f):
	for i in range(0, num + 1):
		f.call(from + (to - from) * i / num)

func begin():
	$Sprite2D.position = Vector2(200, 1000)
	var center = Vector2(570, 200)
	
	# 绘制特效
	tween.tween_interval(0.5)
	tween.tween_method($CirclePainter.draw.bind(center, 170), 0.0, TAU, 0.6)
	tween.tween_method($CirclePainter.draw2.bind(TAU, center), 170.0, 0, 0.8).set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_CUBIC)
	tween.tween_callback(func(): create_tween().tween_method($RoundPainter.draw.bind(center), 0.0, 270.0, 1.2).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC))
	
	
	var radial_shot = func(num: int, r: float):
		var unit_rad	 = TAU / num
		for i in range(0, num):
			var beam = terra_blade_beam.instantiate()
			get_parent().add_child.call_deferred(beam)
			var rad = r + unit_rad * i
			var to = 2000 * Vector2(cos(rad), sin(rad))
			beam.straight_move(center, center + to, 1.2, Tween.EASE_IN, Tween.TRANS_CIRC)
		
	tween.tween_callback(func():
		var t = create_tween()
		t.tween_callback(radial_shot.bind(24, 0))
		t.tween_interval(0.2)
		t.tween_callback(radial_shot.bind(24, TAU / 48))
		)
	tween.tween_property($Sprite2D, "position", Vector2(200, 200), 1).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC)
	tween.parallel().tween_property($Sprite2D, "rotation", TAU + (Vector2(700, 450) - Vector2(200, 200)).angle(), 1).set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CUBIC).as_relative()
	
var seed = PI
func shot_falling_beam(center):
	var beam = terra_blade_beam.instantiate()
	get_parent().add_child.call_deferred(beam)
	var to = 300 * Vector2(cos(seed), sin(seed))
	beam.falling_move(center, to, 800)
	seed = sqrt(seed * seed * seed * 114) / 2
	while seed > 114:
		seed /= TAU
