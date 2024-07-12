extends Node

signal attack_finished
var tween
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


func general_hide():
	tween = create_tween().set_loops()
	tween.tween_callback(self.general_hide_internal.bind()).set_delay(0.01)


func general_hide_internal():
	var tmp = bg.material.get_shader_parameter("alpha_bias")
	if tmp >= 0.9:
		tween.kill()
		bg.hide()
		star_wrath.hide()
	bg.material.set_shader_parameter("alpha_bias", tmp + 0.01)


func start():
	show()
	star_wrath.start()
