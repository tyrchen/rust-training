from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="rust-utils",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["rust_utils"],
    rust_extensions=[RustExtension(
        "rust_utils.rust_utils", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
