extends Fight


func start_fight():
	var manager = self.get_sword_manager()
	if !manager.has_sword():
		return
	var sword = manager.get_and_next_sword()
	sword.connect("attack_finished", self.start_fight.bind())
	await get_tree().create_timer(0.5).timeout
	sword.start()
