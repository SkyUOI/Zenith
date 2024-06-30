extends Node

signal attack_finished


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if get_tree().current_scene == self:
		$StarWrath.start()
	else:
		$StarWrath.hide()


func start():
	$StarWrath.start()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass
