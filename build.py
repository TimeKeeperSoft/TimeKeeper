# A simple script to build TimeKeeper and generate
# packages for different operating systems
#
# USAGE:
#     python ./build.py [win/lin] {nobuild}
#
# (C) 2025 Michail Krasnov <michail383krasnov@mail.ru>

#########################################################
#
# IMPORTS
#
from enum import Enum
import os
import sys
import subprocess
import shutil

#########################################################
#
# Some definitions...
#

# Files to be included in the package directory
#     full path to the file             file name
INCLUDED_FILES = [
    ("README.md",                      "README.md"),
    ("README_ru.md",                   "README_ru.md"),
    ("LICENSE",                        "LICENSE"),
    ("assets/logo.png",                "assets/logo.png"),
]

INCLUDED_EXE = [
    ("target/release/time_keeper.exe", "time_keeper.exe"),
    ("target/release/time_keeper",     "time_keeper"),
]

# Directories to be created in the package directory
CREATED_DIRS: list[str] = ["assets"]

# OS for which TimeKeeper will be built
class OS(Enum):
    Windows = "WINDOWS"
    Linux = "LINUX"

def contains(s1: str, s2: str) -> bool:
    return s2 in s1

class CargoToml:
    def __init__(self):
        with open("./Cargo.toml") as f:
            self.cargo = f.read()

    def get_version(self) -> str:
        strings = self.cargo.split('\n')
        version = "???"
        for s in strings:
            if contains(s, "version"):
                s = s.replace(" ", "")
                s = s.replace("=", "")
                s = s.replace("version", "")
                s = s.replace('"', "")
                version = s
                break
        return version

# Package name
def get_package_name(ver: str, sys: OS) -> str:
    return f"TimeKeeper-v{ver}-{sys.value}-x86_64"

class Package:
    def __init__(self, ver: str, sys: OS):
        self.ver: str = ver
        self.sys: OS  = sys
        self.pkg: sys = get_package_name(ver, sys)

    # Create directory
    def mkdir(self, dir: str):
        if os.path.exists(dir):
            print("Failed to create directory: object is exists")
            sys.exit(1)
        else:
            try:
                os.makedirs(dir)
            except OSError as err:
                print(f"Failed to create directory: {err}")
                sys.exit(1)

    # Create some subdirs
    def create_dirs(self):
        for dir in CREATED_DIRS:
            created_dir = f"{self.pkg}/{dir}"
            print(f"==> Create directory: {created_dir}")
            self.mkdir(created_dir)

    # Remove unneeded temporary directory
    def rmdir(self, dir: str):
        if not os.path.exists(dir):
            print("Failed to remove directory: object doesn't exists")
            sys.exit(1)
        else:
            try:
                shutil.rmtree(dir)
            except OSError as err:
                print(f"Failed to remove directory: {err}")
                sys.exit(1)

    def build(self):
        subprocess.run(["cargo", "build", "--release"])

    def copy_files(self):
        for file in INCLUDED_FILES:
            dest = f"{self.pkg}/{file[1]}"
            print(f"==> Copy file: {file[0]} to {dest}")
            try:
                shutil.copy(file[0], dest)
            except:
                print(f"Failed to copy file {dest}")

        for file in INCLUDED_EXE:
            dest = f"{self.pkg}/{file[1]}"
            print(f"==> Copy EXECUTABLE file: {file[0]} to {dest}")
            try:
                if file[1] == "time_keeper.exe" and self.sys == OS.Windows:
                    shutil.copy(file[0], dest)
                elif file[1] == "time_keeper" and self.sys == OS.Linux:
                    shutil.copy(file[0], dest)
                else:
                    pass
            except:
                print(f"Failed to copy file {dest}")

    def compress(self):
        dest = f"{self.pkg}"
        print(f"==> Create archive {dest}")
        shutil.make_archive(dest, "zip", self.pkg)

    def finish(self):
        print(f"==> Removing directory {self.pkg}")
        self.rmdir(self.pkg)

    def publish(self):
        print("==> Publich new version to crates.io")
        subprocess.run("cargo publish")

    def gen_msi(self):
        print("==> Generating Windows Installer...")
        subprocess.run(f"cargo wix --nocapture --output target/{get_package_name(self.ver, self.sys)}.msi --no-build")

    def gen_appimage(self):
        print("==> Generating AppImage...")
        subprocess.run(["/bin/bash", "./assets/gen_appimage.sh", self.ver])

if __name__ == "__main__":
    version = CargoToml().get_version()

    args: list[str] = sys.argv
    system = OS.Windows

    publish = False
    nobuild = False

    for arg in args:
        if arg == "win":
            system = OS.Windows
        elif arg == "lin":
            system = OS.Linux

        if arg == "pub":
            publish = True

        if arg == "nobuild":
            nobuild = True

    pkg = Package(version, system)

    if not nobuild:
        pkg.build()

    pkg.create_dirs()
    pkg.copy_files()
    pkg.compress()
    pkg.finish()

    if publish:
        pkg.publish()

    if system == OS.Windows:
        pkg.gen_msi()
    elif system == OS.Linux:
        pkg.gen_appimage()
