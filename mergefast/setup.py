#!/usr/bin/env python3

from setuptools import setup, Extension, find_packages\

# https://pypi.org/project/mergefast/
merge_module = Extension(
    "mergefast.core",
    sources=["mergefast/bind.c", "mergefast/merge.c"],
    include_dirs=["mergefast"],
    extra_compile_args=["-O3"]
)

setup(
    name="mergefast",
    version="1.1.3",
    packages=find_packages(),
    ext_modules=[merge_module],
)
