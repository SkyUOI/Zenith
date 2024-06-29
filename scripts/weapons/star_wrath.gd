extends StarWrath

var operations = [func(): self.fall_star()]

var operation_idx = 0


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass  # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	pass


func next_operation():
	if operation_idx >= operations.size():
		attack_finished.emit()
		return
	operations[operation_idx].call()
	operation_idx += 1
