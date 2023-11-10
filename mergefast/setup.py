#!/usr/bin/env python3

from setuptools import find_packages
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="mergefast_rust",
    version="1.1.3",
    packages=find_packages(),
    rust_extensions=[
        RustExtension(
            "mergefast_rust.core",  # Updated module name
            "mergefast_rust/Cargo.toml",  # Adjust path as necessary
            binding=Binding.PyO3
        )
    ],
    include_package_data=True,
    zip_safe=False,
    # Other necessary setup kwargs...
)
