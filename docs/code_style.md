# Code Style

## 1.Tools-based Assistance

- You need to use [gdtoolkit](https://github.com/Scony/godot-gdscript-toolkit) and rustfmt to format the code.You can copy `script/pre-commit` to `.git/hooks` to do that automatically

## 2.Naming Conventions

- For the names of all nodes, we stipulate the use of PascalCase format.

## 3. Correctness Assurance

- For Rust code, first of all, for methods that heavily rely on Godot's scene tree and object functions, since string calling is used, you need to attempt to get these functions and objects in the init function during Debug mode, thereby implementing some automatic testing.
