# How to contribute to this project

First, thank you for being interested in this project and willing to do some contributions.

## Content

- [Code Style](#code-style)
- [Commit Changes](#commit-changes)
- [Tests](#tests)
- [Version Control](#version-control)
- [Other Resource](#other-resource)

## Code Style

Please obey our [Code Style Guide](./docs/code-style.md) to maintain code consistency

## Commit Changes

If you want to commit changes,please obey the following steps:

1. Fork the project from `main`.
2. Create a new branch to develop.
3. Make sure that you code follow our code style.
4. Commit your changes to your fork.
5. Create a Pull Request(PR) to our main branch.

## Tests

For your code,you should write necessary tests to cover your code in both GDScript and Rust.We use Github Action to run the tests

[Rust Test Guide](./docs/rust-test.md)

We will run some Godot scenes to check basic errors.The scenes should be ran is recorded in `test_scene`.In daily development,you can run `script/godot_test.py`to conduct basic testing

## Version Control

Please write your commit message in [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)。

## Other Resource

- [Project Docs](docs/)

---

We are looking forward to your contributions and hope that you can gain joy and sense of achievement
