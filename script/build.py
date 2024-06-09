import os
from utils import msg_system


def main():
    # build rust extension
    os.chdir("../gdrust")
    msg_system("cargo build", "Failed to build rust extension in debug mode")
    msg_system(
        "cargo build --release", "Failed to build rust extension in release mode"
    )
    os.chdir("..")
    print("Rust extension compilation is success")
    print("Build success")


if __name__ == "__main__":
    main()
