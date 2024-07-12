extends Node

signal attack_finished


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if get_tree().current_scene == self:
		self.start()
	else:
		hide()


func hide():
	$StarWrath.hide()
	$Bg.hide()


func show():
	$StarWrath.show()
	$Bg.show()


func start():
	show()
	$StarWrath.start()
