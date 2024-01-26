from setuptools import setup, find_packages
from setuptools.command.install import install
import subprocess
import os


def read_requirements():
    """Read the contents of the requirements.txt file and add 'rust_aha_formulas'."""
    with open("requirements.txt") as f:
        requirements = f.read().splitlines()
    return requirements


class CustomInstallCommand(install):
    def run(self):
        # Build the Rust project to a wheel using Maturin
        subprocess.check_call(
            ["maturin", "build", "--release", "--out", "../target/wheels"]
        )

        # Find the built wheel and install it
        wheel_name = next(
            w
            for w in os.listdir("../target/wheels")
            if "pyprevent" in w and w.endswith(".whl")
        )
        subprocess.check_call(["pip", "install", f"../target/wheels/{wheel_name}"])

        # Navigate back to the Python project directory
        os.chdir(
            "../../"
        )  # Adjust this as needed to return to your Python project root

        # Run the standard setuptools install
        install.run(self)


with open("README.md", encoding="utf-8") as f:
    long_description = f.read()


setup(
    name="pyprevent",
    version="0.1.1",
    packages=find_packages(),
    cmdclass={
        "install": CustomInstallCommand,
    },
    install_requires=read_requirements(),
    description="Python implementation of 2024 AHA PREVENT calculator",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/lhegstrom/PyPrevent/",
    project_urls={
        "Source": "https://github.com/lhegstrom/PyPrevent/",
        "Tracker": "https://github.com/lhegstrom/PyPrevent/issues",
    },
)
