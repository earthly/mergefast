#!/usr/bin/env python3

from setuptools import setup, Extension

pymerge_module = Extension(
    "pymerge.pymerge",
    sources=["pymerge/bind.c", "pymerge/merge.c"],
    extra_compile_args=["-O3"]
)

setup(
    name="pymerge",
    version="1.1",
    ext_modules=[pymerge_module],
)
