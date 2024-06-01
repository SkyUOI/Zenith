extends Sprite2D

signal Enchanted_Sword_Finished # 
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


var finished : bool # 当前状态是否完成
var now_state : int # 当前阶段(对应vectors)
var now_func : String # 当前状态对应函数名

func start(func_name : String, vectors : Array[Vector2]):
	if !finished: # 没完成 就摆烂
		return
	finished = false
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if !finished:
		call(now_func)
	# else : 
		# 应该有个默认状态保持竖直朝下 并上下漂浮?
