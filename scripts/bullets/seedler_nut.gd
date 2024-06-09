extends Area2D

@export var thorns: PackedScene


func _ready():
	$OutScreen.screen_exited.connect(queue_free)


func _process(delta):
	pass


func _on_area_entered(area):
	# thornsCreate()
	# 防止重复伤害
	if area.name == "Player":
		area.get_node("..").hit(10)
		$CollisionShape2D.set_deferred("disabled", true)
