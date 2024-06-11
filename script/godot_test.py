"""
进行Godot测试
"""

import os

from utils import msg_system


def test_scene(path: str):
    msg_system(f"./godot --headless {path}", "Run failed")


os.chdir("..")
with open("test_scene", "r") as files:
    for i in files:
        i = i.strip()
        if not i.endswith(".tscn"):
            print(f"Warning:{i} is not a .tscn file.Ignore")
            continue
        test_scene(i)
