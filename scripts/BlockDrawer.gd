extends BlockDrawer


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_collision_area_entered(area:Area2D):
	self.collision_signal(area)
