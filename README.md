# Fast Unit - Rust Unit Management for Python

Originally created as a faster version of [Unum](https://pypi.org/project/Unum/) 
for use in Python-based FRC robot code.

## Building and Installing for Local Machine

```shell
maturin build --release  # This will generate wheels and put a native library in ./fast_unit/
pip install .  # Installs to your python interpreter
```

## Cross Compilation for RoboRIO (WIP)

```shell
# Add RIO's target triple
rustup target add arm-unknown-linux-gnueabi

# Set cross-compilation environment variables
export PYO3_CROSS_PYTHON_VERSION=3.10
export PYO3_CROSS_LIB_DIR="RIO_ROOT/usr/lib"  # NEED TO FIND THIS

# Build wheels for RIO
maturin build --release --target=arm-unknown-linux-gnueabi 
```