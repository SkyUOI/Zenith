extends Node

@export var terra_blade_beam : PackedScene
var tween = create_tween()

# Called when the node enters the scene tree for the first time.
func _ready():
	begin()

func begin():
	$Sprite2D.position = Vector2(-500, 1000)
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
			beam.straight_move(center, center + to, 1.5, Tween.EASE_IN, Tween.TRANS_CIRC)
		
	tween.tween_callback(radial_shot.bind(24, 0))
	tween.tween_interval(0.2)
	tween.tween_callback(radial_shot.bind(24, TAU / 48))
	
