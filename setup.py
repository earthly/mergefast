#!/usr/bin/env python3

from distutils.core import setup, Extension

setup(
    name = "merge",
    version = "1.0",
    ext_modules = [Extension("merge", ["bind.c", "merge.c"])],
    extra_compile_args = ["-O3"],
    );
