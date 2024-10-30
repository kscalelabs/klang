# mypy: disable-error-code="import-untyped"
#!/usr/bin/env python
"""Setup script for the project."""

import glob
import re
import subprocess

from setuptools import find_packages, setup
from setuptools.command.build_ext import build_ext
from setuptools_rust import Binding, RustExtension

with open("README.md", "r", encoding="utf-8") as f:
    long_description: str = f.read()


with open("pyklang/requirements.txt", "r", encoding="utf-8") as f:
    requirements: list[str] = f.read().splitlines()


with open("pyklang/requirements-dev.txt", "r", encoding="utf-8") as f:
    requirements_dev: list[str] = f.read().splitlines()


with open("pyklang/__init__.py", "r", encoding="utf-8") as fh:
    version_re = re.search(r"^__version__ = \"([^\"]*)\"", fh.read(), re.MULTILINE)
assert version_re is not None, "Could not find version in pyklang/__init__.py"
version: str = version_re.group(1)

package_data = [f"pyklang/{name}" for name in ("py.typed", "requirements.txt", "requirements-dev.txt")]
package_data.append("Cargo.toml")
for ext in ("pyi", "rs", "toml", "so"):
    package_data.extend(glob.iglob(f"pyklang/**/*.{ext}", recursive=True))


class RustBuildExt(build_ext):
    def run(self) -> None:
        # Run the stub generator
        subprocess.run(["cargo", "run", "--bin", "stub_gen"], check=True)
        # Call the original build_ext command
        super().run()


setup(
    name="pyklang",
    version=version,
    description="Python interface for Klang",
    author="K-Scale Labs",
    url="https://github.com/kscalelabs/klang",
    rust_extensions=[
        RustExtension(
            target="pyklang.bindings",
            path="pyklang/Cargo.toml",
            binding=Binding.PyO3,
        ),
    ],
    setup_requires=["setuptools-rust"],
    zip_safe=False,
    long_description=long_description,
    long_description_content_type="text/markdown",
    python_requires=">=3.11",
    install_requires=requirements,
    extras_require={"dev": requirements_dev},
    include_package_data=True,
    package_data={"pyklang": package_data},
    packages=find_packages(include=["pyklang"]),
    cmdclass={"build_ext": RustBuildExt},
)
