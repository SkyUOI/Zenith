extends Node

signal attack_finished
@onready var star_wrath = $StarWrath
@onready var bg = $Bg


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	if get_tree().current_scene == self:
		self.start()
	else:
		hide()


func hide():
	star_wrath.hide()
	bg.hide()


func show():
	star_wrath.show()
	bg.show()


func start():
	show()
	star_wrath.start()
