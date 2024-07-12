extends StarWrath

@export var star_wrath_origin: PackedScene
var operations = [func(): self.fall_star_process()]
var operation_idx = 0


func next_operation():
	if operation_idx >= operations.size():
		$"..".attack_finished.emit()
		return
	operations[operation_idx].call()
	operation_idx += 1


func fall_star_process():
	for i in range(10):
		self.fall_star()
		await get_tree().create_timer(randf_range(1.0, 2.0)).timeout
	next_operation()


func laser():
	pass
