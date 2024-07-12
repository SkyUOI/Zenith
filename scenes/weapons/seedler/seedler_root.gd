extends Node

signal attack_finished
@onready var seedler = $Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass  # Replace with function body.


func start():
	seedler.start()
