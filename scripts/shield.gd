extends Area2D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	#print(self.modulate.a)
	pass  # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func general_hide():
	var tween = create_tween()
	var cp = self.modulate
	cp.a = 0
	tween.tween_property(self, "modulate", cp, 0.15)
	tween.tween_callback(self.hide)


func show_wrapper():
	self.modulate.a = 1
	show()
