import os

def msg_system(cmd, error_msg):
    if os.system(cmd) != 0:
        print(error_msg)
        exit(1)

def main():
    # build rust extension
    os.chdir("../gdrust")
    msg_system("cargo build", "Failed to build rust extension in debug mode")
    msg_system("cargo build --release", "Failed to build rust extension in release mode")
    os.chdir("..")
    print("Rust extension compilation is success")
    print("Build success")

if __name__ == "__main__":
    main()