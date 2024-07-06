extends VBoxContainer

@export var multi_join: PackedScene
@export var multi_set_up: PackedScene


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass  # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func _on_join_pressed() -> void:
	get_tree().change_scene_to_packed(multi_join)


func _on_setup_pressed() -> void:
	get_tree().change_scene_to_packed(multi_set_up)
