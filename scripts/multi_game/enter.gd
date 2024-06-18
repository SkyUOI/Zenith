extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_pressed() -> void:
	Global.connected_ip = $"../Input/IPBox/IP".text
	Global.player_name = $"../Input/PlayerBox/Player".text
	if Global.connected_ip.is_empty():
		$WrongDialog.dialog_text = "Ip address is empty"
		$WrongDialog.popup_centered()
		return
	if Global.player_name.is_empty():
		$WrongDialog.dialog_text = "Player name is empty"
		$WrongDialog.popup_centered()
		return
	get_tree().change_scene_to_file("res://scenes/fight.tscn")
